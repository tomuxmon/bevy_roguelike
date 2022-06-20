pub mod components;
// pub mod dragable_ui;
pub mod events;
pub mod resources;
pub mod systems;

use crate::components::*;
use crate::events::*;
use bevy::ecs::schedule::StateData;
use bevy::log;
use bevy::prelude::*;
use bevy::render::camera::Camera2d;
use bevy::utils::HashSet;
use bevy_asset_ron::RonAssetPlugin;
use bevy_inventory_ui::InventoryUiAssets;
use bevy_inventory_ui::InventoryUiPlugin;
use bevy_tweening::TweeningPlugin;
use map_generator::*;
use rand::prelude::*;
use resources::*;
use std::marker::PhantomData;
use std::ops::Range;
use systems::actor_stats::*;
use systems::camera::*;
use systems::fov::*;
use systems::input::*;
use systems::inventory::*;
use systems::map::*;
use systems::render::*;
use systems::turns::*;

// TODO: review all `as T` casting

pub struct RoguelikePlugin<T> {
    /// Asset loading happens in this state. When it finishes it transitions to [`RoguelikePlugin::state_construct`]
    pub state_asset_load: T,
    pub state_construct: T,
    pub state_running: T,
}

pub trait StateNext: StateData {
    fn next(&self) -> Option<Self>;
}
pub trait StateSetNext {
    fn set_next(&mut self);
}
impl<T: StateNext> StateSetNext for State<T> {
    fn set_next(&mut self) {
        let current = self.current();
        if let Some(next) = current.next() {
            if let Err(error) = self.set(next) {
                bevy::log::error!("Error setting next state. {}", error);
            }
        } else {
            bevy::log::error!("no next state for {:?}.", current);
        }
    }
}

#[derive(Default)]
pub struct AssetsLoading(pub Vec<HandleUntyped>);

#[derive(Debug)]
pub struct MapEntities {
    map_id: Entity,
    enemies_id: Entity,
    items_id: Entity,
}

// TODO: instead of after / before  use labels: https://bevy-cheatbook.github.io/programming/system-order.html#labels

impl<T: StateNext> Plugin for RoguelikePlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_plugin(TweeningPlugin {})
            .add_plugin(InventoryUiPlugin::<_, RogueItemType, InventoryAssets> {
                state_running: self.state_running.clone(),
                phantom_1: PhantomData {},
                phantom_2: PhantomData {},
            })
            .add_plugin(RonAssetPlugin::<ItemTemplate>::new(&["item.ron"]))
            .add_plugin(RonAssetPlugin::<ActorTemplate>::new(&["actor.ron"]))
            .add_plugin(RonAssetPlugin::<MapTheme>::new(&["maptheme.ron"]))
            .add_plugin(RonAssetPlugin::<InventoryTheme>::new(&[
                "inventorytheme.ron",
            ]))
            .insert_resource(AssetsLoading::default())
            .add_startup_system(Self::rogue_setup)
            .add_startup_system(setup_camera)
            .add_system_set(
                SystemSet::on_update(self.state_asset_load.clone())
                    .with_system(Self::check_assets_ready),
            )
            .add_system_set(
                SystemSet::on_enter(self.state_construct.clone()).with_system(Self::create_map),
            )
            .add_system_set(
                SystemSet::on_update(self.state_running.clone())
                    .with_system(input_player::<RogueItemType>.after(gather_action_points))
                    .with_system(input_fov_rand.after(gather_action_points))
                    .with_system(render_body)
                    .with_system(actors_fill_text_info.after(stats_recompute::<RogueItemType>))
                    .with_system(render_equiped_item::<RogueItemType>)
                    .with_system(unrender_unequiped_items)
                    .with_system(render_hud_health_bar)
                    .with_system(attributes_update_action_points)
                    .with_system(attributes_update_hit_points)
                    .with_system(attributes_update_field_of_view)
                    .with_system(stats_recompute::<RogueItemType>)
                    .with_system(gather_action_points)
                    .with_system(turn_end_now_gather.after(gather_action_points))
                    .with_system(act)
                    .with_system(attack.after(act))
                    .with_system(item_fill_text_info::<RogueItemType>)
                    .with_system(pick_up_items::<RogueItemType>)
                    .with_system(drop_item::<RogueItemType>)
                    .with_system(
                        equip_owned_add::<RogueItemType>
                            .after(pick_up_items::<RogueItemType>)
                            .after(drop_item::<RogueItemType>),
                    )
                    .with_system(
                        equip_owned_remove::<RogueItemType>
                            .after(pick_up_items::<RogueItemType>)
                            .after(drop_item::<RogueItemType>),
                    )
                    .with_system(toggle_inventory_open_event_send::<RogueItemType>)
                    .with_system(spend_ap.after(act))
                    .with_system(try_move.after(act).after(spend_ap))
                    .with_system(apply_position_to_transform.after(try_move))
                    .with_system(apply_hp_modify.after(act).after(attack).after(spend_ap))
                    .with_system(death_read::<RogueItemType>.after(apply_hp_modify))
                    .with_system(idle_rest.after(apply_hp_modify))
                    .with_system(camera_set_focus_player)
                    .with_system(camera_focus_smooth.after(camera_set_focus_player))
                    .with_system(field_of_view_recompute)
                    .with_system(field_of_view_set_visibility.after(field_of_view_recompute)),
            )
            .add_system_set(
                SystemSet::on_exit(self.state_running.clone()).with_system(Self::cleanup_map),
            )
            .register_type::<Vector2D>()
            .register_type::<RenderInfo>()
            .register_type::<RenderInfoEquiped>()
            .register_type::<MapTile>()
            .register_type::<Attributes>()
            .register_type::<AttributeType>()
            .register_type::<ActionPoints>()
            .register_type::<ActionPointsDirty>()
            .register_type::<HitPoints>()
            .register_type::<HitPointsDirty>()
            .register_type::<TurnState>()
            .register_type::<ModifyHP>()
            .register_type::<Team>()
            .register_type::<MovingPlayer>()
            .register_type::<MovingRandom>()
            .register_type::<MovingFovRandom>()
            .register_type::<FieldOfView>()
            .register_type::<FieldOfViewDirty>()
            .register_type::<DamageKind>()
            .register_type::<AttributeMultiplier>()
            .register_type::<Formula>()
            .register_type::<Rate>()
            .register_type::<ActionCost>()
            .register_type::<Damage>()
            .register_type::<Protect>()
            .register_type::<Resist>()
            .register_type::<Protection>()
            .register_type::<Resistance>()
            .register_type::<Evasion>()
            .register_type::<Block>()
            .register_type::<Evasion>()
            .register_type::<StatsComputed>()
            .register_type::<StatsComputedDirty>()
            .register_type::<Quality>()
            .register_type::<HashSet<IVec2>>()
            .register_type::<Vec<DamageKind>>()
            .register_type::<Vec<Protect>>()
            .register_type::<HashSet<Resist>>()
            .register_type::<Range<i32>>()
            .add_event::<SpendAPEvent>()
            .add_event::<AttackEvent>()
            .add_event::<MoveEvent>()
            .add_event::<ActEvent>()
            .add_event::<IdleEvent>()
            .add_event::<CameraFocusEvent>()
            .add_event::<DeathEvent>();

        log::info!("Loaded Roguelike Plugin");
    }
}

impl<T: StateNext> RoguelikePlugin<T> {
    fn check_assets_ready(
        mut commands: Commands,
        server: Res<AssetServer>,
        loading: Res<AssetsLoading>,
        mut state: ResMut<State<T>>,
    ) {
        use bevy::asset::LoadState;

        match server.get_group_load_state(loading.0.iter().map(|h| h.id)) {
            LoadState::Failed => {
                bevy::log::error!("Asset load failed. Check preceding warnings for more info. Transitioning to next state anyway (feeling adventurous).");
                state.set_next();
            }
            LoadState::Loaded => {
                state.set_next();
                commands.remove_resource::<AssetsLoading>();
                // (note: if you don't have any other handles to the assets
                // elsewhere, they will get unloaded after this)
            }
            _ => {
                bevy::log::info!("loading assets...");
            }
        }
    }

    fn cleanup_map(map_id: Res<MapEntities>, mut cmd: Commands) {
        cmd.entity(map_id.map_id).despawn_recursive();
        cmd.entity(map_id.enemies_id).despawn_recursive();
        // TODO: leave player items
        cmd.entity(map_id.items_id).despawn_recursive();
        cmd.remove_resource::<MapEntities>();
    }

    fn rogue_setup(
        asset_server: Res<AssetServer>,
        mut state: ResMut<State<T>>,
        mut loading: ResMut<AssetsLoading>,
    ) {
        if let Ok(handles) = asset_server.load_folder("") {
            loading.0.extend(handles);
        }
        // NOTE: transitioning from Setup to AssetLoad
        state.set_next();
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_map(
        mut cmd: Commands,
        mut state: ResMut<State<T>>,
        map_options: Option<Res<MapOptions>>,
        asset_server: Res<AssetServer>,
        map_themes: Res<Assets<MapTheme>>,
        item_templates: Res<Assets<ItemTemplate>>,
        inventory_themes: Res<Assets<InventoryTheme>>,
        actor_templates: Res<Assets<ActorTemplate>>,
        mut cameras: Query<&mut Transform, With<Camera2d>>,
    ) {
        let options = match map_options {
            None => MapOptions::default(), // If no options is set we use the default one
            Some(o) => o.clone(),
        };

        // max u64: 18_446_744_073_709_551_615
        // let mut rng = StdRng::seed_from_u64(54155745465);
        let trng = thread_rng();
        let mut rng = StdRng::from_rng(trng).expect("Could not construct StdRng using ThreadRng");

        let map_generator = RandomMapGenerator {};
        let map = map_generator.gen(&mut rng, options.map_size);
        let info = MapInfo::from_map(&map, &mut rng);

        #[cfg(feature = "debug")]
        log::info!("{}", map.to_colorized_string());
        #[cfg(feature = "debug")]
        log::info!("{}", info.to_colorized_string());

        for mut c in cameras.iter_mut() {
            let z = c.translation.z;
            let new_pos = options.to_world_position(info.camera_focus).extend(z);
            c.translation = new_pos;
        }

        let inventory_themes: Vec<_> = inventory_themes.iter().map(|(_, it)| it).collect();
        let inventory_theme = inventory_themes[rng.gen_range(0..inventory_themes.len())];
        cmd.insert_resource(InventoryAssets {
            slot: asset_server.load(inventory_theme.slot.as_str()),
            head_wear: asset_server.load(inventory_theme.head_wear.as_str()),
            body_wear: asset_server.load(inventory_theme.body_wear.as_str()),
            main_hand_gear: asset_server.load(inventory_theme.main_hand_gear.as_str()),
            off_hand_gear: asset_server.load(inventory_theme.off_hand_gear.as_str()),
            finger_wear: asset_server.load(inventory_theme.finger_wear.as_str()),
            neck_wear: asset_server.load(inventory_theme.neck_wear.as_str()),
            feet_wear: asset_server.load(inventory_theme.feet_wear.as_str()),
        });
        cmd.insert_resource(InventoryUiAssets {
            slot: asset_server.load(inventory_theme.slot.as_str()),
            hover_cursor_image: asset_server.load("sprites/gui/tooltip/cursor.png"),
            font: asset_server.load("fonts/pixeled.ttf"),
        });

        let map_themes: Vec<_> = map_themes.iter().map(|(_, it)| it).collect();
        let map_theme = map_themes[rng.gen_range(0..map_themes.len())];
        let map_id = cmd
            .spawn()
            .insert(Name::new("RogueMap"))
            .insert(Transform::default())
            .insert(GlobalTransform::default())
            .with_children(|rogue_map| {
                for (pt, tile) in map.enumerate() {
                    let texture = asset_server.load(
                        match tile {
                            Tile::Wall => {
                                map_theme.wall[rng.gen_range(0..map_theme.wall.len())].clone()
                            }
                            Tile::Floor => {
                                map_theme.floor[rng.gen_range(0..map_theme.floor.len())].clone()
                            }
                        }
                        .as_str(),
                    );
                    rogue_map
                        .spawn()
                        .insert(Name::new(format!("Tile {}", pt)))
                        .insert(Vector2D::from(pt))
                        .insert(match tile {
                            Tile::Wall => MapTile { is_passable: false },
                            Tile::Floor => MapTile { is_passable: true },
                        })
                        .insert(RenderInfo {
                            texture,
                            cosmetic_textures: vec![],
                            z: 0.,
                        });
                }
            })
            .id();

        let item_templates: Vec<_> = item_templates.iter().map(|(_, it)| it).collect();
        let items_id = cmd
            .spawn()
            .insert(Name::new("Items"))
            .insert(Transform::default())
            .insert(GlobalTransform::default())
            .with_children(|cb| {
                for ipt in info.item_spawns.clone() {
                    let template = item_templates[rng.gen_range(0..item_templates.len())];
                    let quality = Quality::roll(&mut rng);
                    let mut ecmd = cb.spawn();
                    ecmd.insert(Vector2D::from(ipt));
                    spawn_item(
                        &mut ecmd,
                        asset_server.clone(),
                        template,
                        &quality,
                        &mut rng,
                    );
                }
            })
            .id();

        if let Some(player_template) = actor_templates.get("actors/human.actor.ron") {
            let team_player = 1;
            cmd.spawn()
                .insert(MovingPlayer {})
                .insert_bundle(Actor::new(
                    asset_server.clone(),
                    player_template,
                    team_player,
                    info.player_start,
                ));
        } else {
            bevy::log::error!("human actor template not found");
        }

        let actor_templates: Vec<_> = actor_templates.iter().map(|(_, it)| it).collect();
        let enemies_id = cmd
            .spawn()
            .insert(Name::new("Enemies"))
            .insert(Transform::default())
            .insert(GlobalTransform::default())
            .with_children(|enms| {
                for mpt in info.monster_spawns.clone() {
                    let monster_template = actor_templates[rng.gen_range(0..actor_templates.len())];
                    let team_monster = 1 + rng.gen_range(2..4);
                    enms.spawn()
                        .insert(MovingFovRandom {})
                        .insert_bundle(Actor::new(
                            asset_server.clone(),
                            monster_template,
                            team_monster,
                            mpt,
                        ));
                }
            })
            .id();

        cmd.insert_resource(map);
        cmd.insert_resource(info);
        cmd.insert_resource(rng);

        cmd.insert_resource(MapEntities {
            map_id,
            enemies_id,
            items_id,
        });

        state.set_next();
    }
}

fn setup_camera(mut cmd: Commands) {
    cmd.spawn_bundle(OrthographicCameraBundle::new_2d());
    cmd.spawn_bundle(UiCameraBundle::default());
}
