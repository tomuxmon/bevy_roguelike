pub mod components;
pub mod events;
pub mod resources;
pub mod systems;

use crate::components::*;
use crate::events::*;
use bevy::log;
use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_inventory_ui::InventoryUiAssets;
use bevy_inventory_ui::InventoryUiPlugin;
use bevy_roguelike_combat::*;
use bevy_roguelike_states::AppState;
use bevy_tweening::TweeningPlugin;
use map_generator::*;
use rand::prelude::*;
use resources::*;
use std::ops::Range;
use systems::action::*;
use systems::actor_stats::*;
use systems::camera::*;
use systems::fov::*;
use systems::input::*;
use systems::inventory::*;
use systems::map::*;
use systems::render::*;
use systems::turns::*;

pub struct RoguelikePlugin {}

#[derive(Resource, Default)]
pub struct AssetsLoading(pub Vec<HandleUntyped>);

#[derive(Resource, Debug)]
pub struct MapEntities {
    map_id: Entity,
    enemies_id: Entity,
    items_id: Entity,
}

// TODO: instead of after / before  use labels: https://bevy-cheatbook.github.io/programming/system-order.html#labels

impl Plugin for RoguelikePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TweeningPlugin {})
            .add_plugin(InventoryUiPlugin::<RogueItemType, InventoryAssets>::default())
            .add_plugin(RoguelikeCombatPlugin::<RogueDamageKind, RogueAttributeType>::default())
            .add_plugin(RonAssetPlugin::<ItemTemplate>::new(&["item.ron"]))
            .add_plugin(RonAssetPlugin::<ActorTemplate>::new(&["actor.ron"]))
            .add_plugin(RonAssetPlugin::<MapTheme>::new(&["maptheme.ron"]))
            .add_plugin(RonAssetPlugin::<InventoryTheme>::new(&[
                "inventorytheme.ron",
            ]))
            .add_plugin(RonAssetPlugin::<CombatSettings>::new(&["combat.ron"]))
            .insert_resource(AssetsLoading::default())
            .add_startup_system(Self::rogue_setup)
            .add_startup_system(setup_camera)
            .add_system(Self::check_assets_ready.run_if(in_state(AppState::AssetLoad)))
            .add_system(Self::create_map.in_schedule(OnEnter(AppState::Construct)))
            .add_systems(
                (
                    apply_position_to_transform.run_if(in_state(AppState::InGame)),
                    camera_set_focus_player.run_if(in_state(AppState::InGame)),
                    field_of_view_recompute.run_if(in_state(AppState::InGame)),
                )
                    .in_base_set(CoreSet::First),
            )
            .add_systems(
                (
                    gather_action_points.run_if(in_state(AppState::InGame)),
                    turn_end_now_gather.run_if(in_state(AppState::InGame)),
                    stats_recompute::<RogueItemType>.run_if(in_state(AppState::InGame)),
                    attributes_update_field_of_view.run_if(in_state(AppState::InGame)),
                    field_of_view_set_visibility.run_if(in_state(AppState::InGame)),
                    actors_fill_text_info.run_if(in_state(AppState::InGame)),
                    item_fill_text_info::<RogueItemType>.run_if(in_state(AppState::InGame)),
                    camera_focus_smooth.run_if(in_state(AppState::InGame)),
                    equip_owned_add::<RogueItemType>.run_if(in_state(AppState::InGame)),
                    equip_owned_remove::<RogueItemType>.run_if(in_state(AppState::InGame)),
                    toggle_inventory_open_event_send::<RogueItemType>
                        .run_if(in_state(AppState::InGame)),
                )
                    .in_base_set(CoreSet::PreUpdate),
            )
            .add_systems(
                (
                    input_player::<RogueItemType>.run_if(in_state(AppState::InGame)),
                    input_fov_rand.run_if(in_state(AppState::InGame)),
                    render_body.run_if(in_state(AppState::InGame)),
                    render_equiped_item::<RogueItemType>.run_if(in_state(AppState::InGame)),
                    unrender_unequiped_items.run_if(in_state(AppState::InGame)),
                    render_hud_health_bar.run_if(in_state(AppState::InGame)),
                    act.run_if(in_state(AppState::InGame)),
                    action_completed.run_if(in_state(AppState::InGame)),
                    try_move.after(act).run_if(in_state(AppState::InGame)),
                )
                    .in_base_set(CoreSet::Update),
            )
            .add_systems(
                (
                    pick_up_items::<RogueItemType>.run_if(in_state(AppState::InGame)),
                    drop_item::<RogueItemType>.run_if(in_state(AppState::InGame)),
                    death_read::<RogueItemType>.run_if(in_state(AppState::InGame)),
                )
                    .in_base_set(CoreSet::PostUpdate),
            )
            .add_system(Self::cleanup_map.in_schedule(OnExit(AppState::InGame)))
            .register_type::<Vector2D>()
            .register_type::<RenderInfo>()
            .register_type::<RenderInfoEquiped>()
            .register_type::<MapTile>()
            .register_type::<TurnState>()
            .register_type::<Team>()
            .register_type::<MovingPlayer>()
            .register_type::<MovingRandom>()
            .register_type::<MovingFovRandom>()
            .register_type::<FieldOfView>()
            .register_type::<FieldOfViewDirty>()
            .register_type::<Quality>()
            .register_type::<HashSet<IVec2>>()
            .register_type::<Range<i32>>()
            .add_event::<MoveEvent>()
            .add_event::<ActEvent>()
            .add_event::<CameraFocusEvent>();

        log::info!("Loaded Roguelike Plugin");
    }
}

impl RoguelikePlugin {
    fn check_assets_ready(
        mut commands: Commands,
        server: Res<AssetServer>,
        loading: Res<AssetsLoading>,
        mut state: ResMut<NextState<AppState>>,
    ) {
        use bevy::asset::LoadState;

        match server.get_group_load_state(loading.0.iter().map(|h| h.id())) {
            LoadState::Failed => {
                bevy::log::error!("Asset load failed. Check preceding warnings for more info. Transitioning to next state anyway (feeling adventurous).");
                state.set(AppState::Construct);
            }
            LoadState::Loaded => {
                bevy::log::info!("Assets loaded.");
                state.set(AppState::Construct);
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
        mut state: ResMut<NextState<AppState>>,
        mut loading: ResMut<AssetsLoading>,
    ) {
        log::info!("Loading assets...");

        #[cfg(not(target_arch = "wasm32"))]
        match asset_server.load_folder("") {
            Ok(handles) => loading.0.extend(handles),
            Err(err) => bevy::log::error!("{}", err),
        }
        #[cfg(target_arch = "wasm32")]
        {
            let asset_files: Vec<&str> = vec_walk_dir::vec_walk_dir!("assets");
            for file in asset_files.into_iter() {
                loading.0.push(asset_server.load_untyped(file));
            }
        }
        state.set(AppState::AssetLoad);
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_map(
        mut cmd: Commands,
        mut state: ResMut<NextState<AppState>>,
        map_options: Option<Res<MapOptions>>,
        asset_server: Res<AssetServer>,
        map_themes: Res<Assets<MapTheme>>,
        item_templates: Res<Assets<ItemTemplate>>,
        inventory_themes: Res<Assets<InventoryTheme>>,
        combat_settings: Res<Assets<CombatSettings>>,
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
        log::trace!("{}", map.to_colorized_string());
        #[cfg(feature = "debug")]
        log::info!("{}", info.to_colorized_string());

        for mut c in cameras.iter_mut() {
            let z = c.translation.z;
            let new_pos = options.to_world_position(info.camera_focus).extend(z);
            c.translation = new_pos;
        }

        let inventory_themes: Vec<_> = inventory_themes.iter().map(|(_, it)| it).collect();
        bevy::log::info!("inventory theme count: {}", inventory_themes.len());

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
            .spawn((SpatialBundle::default(), Name::new("RogueMap")))
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
                    rogue_map.spawn((
                        Name::new(format!("Tile {}", pt)),
                        // Vector2D::from(pt),
                        RenderInfo {
                            texture,
                            cosmetic_textures: vec![],
                            z: 0.,
                        },
                        match tile {
                            Tile::Wall => MapTile { is_passable: false },
                            Tile::Floor => MapTile { is_passable: true },
                        },
                    ));
                }
            })
            .id();

        let item_templates: Vec<_> = item_templates.iter().map(|(_, it)| it).collect();
        let items_id = cmd
            .spawn((SpatialBundle::default(), Name::new("Items")))
            .with_children(|cb| {
                for ipt in info.item_spawns.clone() {
                    let template = item_templates[rng.gen_range(0..item_templates.len())];
                    let quality = Quality::roll(&mut rng);
                    let mut ecmd = cb.spawn(Vector2D::from(ipt));
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

        let combat_settings: Vec<_> = combat_settings.iter().map(|(_, it)| it).collect();
        bevy::log::info!("combat settings count: {}", combat_settings.len());
        let combat_settings = combat_settings[0];

        if let Some(player_template) =
            actor_templates.get(&asset_server.load("actors/human.actor.ron"))
        {
            let team_player = 1;
            cmd.spawn((
                Actor::new(
                    asset_server.clone(),
                    player_template,
                    combat_settings,
                    team_player,
                    info.player_start,
                ),
                MovingPlayer {},
            ));
        } else {
            bevy::log::error!("human actor template not found");
        }

        let actor_templates: Vec<_> = actor_templates.iter().map(|(_, it)| it).collect();
        let enemies_id = cmd
            .spawn((SpatialBundle::default(), Name::new("Enemies")))
            .with_children(|enms| {
                for mpt in info.monster_spawns.clone() {
                    let monster_template = actor_templates[rng.gen_range(0..actor_templates.len())];
                    let team_monster = 1 + rng.gen_range(2..4);
                    enms.spawn((
                        Actor::new(
                            asset_server.clone(),
                            monster_template,
                            combat_settings,
                            team_monster,
                            mpt,
                        ),
                        MovingFovRandom {},
                    ));
                }
            })
            .id();

        cmd.insert_resource(RogueMap(map));
        cmd.insert_resource(info);
        cmd.insert_resource(RogueRng(rng));

        cmd.insert_resource(MapEntities {
            map_id,
            enemies_id,
            items_id,
        });

        bevy::log::info!("Construct completed. Entering Game.");

        state.set(AppState::InGame);
    }
}

fn setup_camera(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle::default());
}
