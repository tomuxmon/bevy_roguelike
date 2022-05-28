pub mod components;
pub mod dragable_ui;
pub mod events;
pub mod quality;
pub mod resources;
pub mod systems;

use std::ops::Range;

use crate::components::*;
use crate::dragable_ui::*;
use crate::events::*;
use bevy::ecs::schedule::StateData;
use bevy::log;
use bevy::prelude::*;
use bevy::reflect::TypeRegistry;
use bevy::render::camera::Camera2d;
use bevy::utils::HashSet;
use bevy_easings::*;
use map_generator::*;
use rand::prelude::*;
use resources::*;
use systems::actor_stats::*;
use systems::camera::*;
use systems::fov::*;
use systems::inventory::*;
use systems::map::*;
use systems::render::*;
use systems::turns::*;

pub struct RoguelikePlugin<T> {
    /// Asset loading happens in this state. When it finishes it transitions to [`RoguelikePlugin::game_construct_state`]
    pub asset_load_state: T,
    pub game_construct_state: T,
    pub running_state: T,
}

pub trait StateNext: StateData {
    fn next(&self) -> Option<Self>;
}

#[derive(Default)]
pub struct AssetsLoading(pub Vec<HandleUntyped>);

#[derive(Debug)]
pub struct MapEntities {
    map_id: Entity,
    enemies_id: Entity,
}

impl<T: StateNext> Plugin for RoguelikePlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_plugin(EasingsPlugin {})
            .insert_resource(AssetsLoading::default())
            .add_startup_system(setup_camera)
            .add_system_set(
                SystemSet::on_update(self.asset_load_state.clone())
                    .with_system(Self::check_assets_ready),
            )
            .add_system_set(
                SystemSet::on_enter(self.game_construct_state.clone())
                    .with_system(Self::create_map),
            )
            .add_system_to_stage(CoreStage::First, ui_drag_interaction)
            .add_system_set(
                SystemSet::on_update(self.running_state.clone())
                    .with_system(ui_apply_drag_pos)
                    .with_system(render_body)
                    .with_system(render_hud_health_bar)
                    .with_system(attributes_update_action_points)
                    .with_system(attributes_update_hit_points)
                    .with_system(attributes_update_field_of_view)
                    .with_system(gather_action_points)
                    .with_system(turn_end_now_gather.after(gather_action_points))
                    .with_system(act)
                    .with_system(attack.after(act))
                    .with_system(pick_up_items)
                    .with_system(toggle_inventory_open)
                    .with_system(equipment_update)
                    .with_system(inventory_update)
                    .with_system(drop_item)
                    .with_system(spend_ap.after(act))
                    .with_system(do_move.after(act).after(spend_ap))
                    .with_system(apply_position_to_transform.after(do_move))
                    .with_system(apply_hp_modify.after(act).after(spend_ap))
                    .with_system(idle_rest.after(apply_hp_modify))
                    .with_system(camera_set_focus_player)
                    .with_system(camera_focus_smooth.after(camera_set_focus_player))
                    .with_system(field_of_view_recompute)
                    .with_system(field_of_view_set_visibility.after(field_of_view_recompute)),
            )
            .add_system_set(
                SystemSet::on_exit(self.running_state.clone()).with_system(Self::cleanup_map),
            )
            .register_type::<Vector2D>()
            .register_type::<RenderInfo>()
            .register_type::<MapTile>()
            .register_type::<Attributes>()
            .register_type::<AttributeType>()
            .register_type::<ActionPoints>()
            .register_type::<HitPoints>()
            .register_type::<TurnState>()
            .register_type::<ModifyHP>()
            .register_type::<Team>()
            .register_type::<MovingPlayer>()
            .register_type::<MovingRandom>()
            .register_type::<MovingFovRandom>()
            .register_type::<FieldOfView>()
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
            .register_type::<ItemType>()
            .register_type::<ItemEquipSlot>()
            .register_type::<ItemDisplaySlot>()
            .register_type::<EquipmentDisplay>()
            .register_type::<Equipment>()
            .register_type::<DragableUI>()
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
            .add_event::<PickUpItemEvent>()
            .add_event::<DropItemEvent>()
            .add_event::<CameraFocusEvent>();

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
                bevy::log::error!("Asset load failed. Validate path's.");
            }
            LoadState::Loaded => {
                if let Some(next_state) = state.current().next() {
                    state.set(next_state).unwrap();
                } else {
                    bevy::log::error!("no next state.");
                }
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
        cmd.remove_resource::<MapEntities>();
    }

    pub fn create_map(
        mut cmd: Commands,
        mut state: ResMut<State<T>>,
        map_options: Option<Res<MapOptions>>,
        map_assets: Res<MapAssets>,
        player_assets: Res<PlayerAssets>,
        enemy_assets: Res<EnemyAssets>,
        item_assets: Res<ItemAssets>,
        type_registry: Res<TypeRegistry>,
        prefabs: Res<Prefabs>,
        dyn_scenes: Res<Assets<DynamicScene>>,
        // mut scene_spawner: ResMut<SceneSpawner>,
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
        let mut team_map = TeamMap::empty(options.map_size);

        #[cfg(feature = "debug")]
        log::info!("{}", map.to_colorized_string());
        #[cfg(feature = "debug")]
        log::info!("{}", info.to_colorized_string());

        for mut c in cameras.iter_mut() {
            let z = c.translation.z;
            let new_pos = options.to_world_position(info.camera_focus).extend(z);
            c.translation = new_pos;
        }

        // TODO: experiment with dynamic scene
        let mut prefab_world = World::new();

        prefab_world.spawn().insert_bundle((
            ItemType::MainHand,
            Damage {
                kind: DamageKind::Slash,
                amount: 5..9,
                amount_multiplier: Formula::new(vec![AttributeMultiplier {
                    multiplier: 100,
                    attribute: AttributeType::Strength,
                }]),
                hit_cost: ActionCost {
                    cost: 128,
                    cost_multiplier: Formula::new(vec![AttributeMultiplier {
                        multiplier: 100,
                        attribute: AttributeType::Dexterity,
                    }]),
                },
                hit_chance: Rate {
                    amount: 90,
                    multiplier: Formula::new(vec![AttributeMultiplier {
                        multiplier: 110,
                        attribute: AttributeType::Dexterity,
                    }]),
                },
            },
            Protection::new(vec![Protect {
                amount: 2,
                kind: DamageKind::Slash,
                amount_multiplier: Formula::new(vec![AttributeMultiplier {
                    multiplier: 100,
                    attribute: AttributeType::Dexterity,
                }]),
            }]),
        ));

        let scene = DynamicScene::from_world(&prefab_world, &*type_registry);
        info!("{}", scene.serialize_ron(&*type_registry).unwrap());

        info!("len {}", dyn_scenes.len());

        if let Some(scn) = dyn_scenes.get(prefabs.ron_scene.clone()) {
            // scn.write_to_world(world, entity_map);

            for dyn_entity in scn.entities.iter() {
                // TODO: first component should be a marker component describinf if it is an item or something else.
                // let first = dyn_entity.components.iter().nth(0);

                // let mut item_cmd = cmd.spawn();
                // let item = item_cmd.insert(Name::new("item")).id();
                for reflect_component in dyn_entity.components.iter() {
                    // let comp = reflect_component.downcast::<dyn Component>();

                    //item_cmd.insert(reflect_component);
                    info!("component: {:?}", reflect_component);
                }
            }
        }

        let map_id = cmd
            .spawn()
            .insert(Name::new("RogueMap"))
            .insert(Transform::default())
            .insert(GlobalTransform::default())
            .with_children(|rogue_map| {
                for (pt, tile) in map.enumerate() {
                    let texture = match tile {
                        Tile::Wall => {
                            map_assets.wall[rng.gen_range(0..map_assets.wall.len())].clone()
                        }
                        Tile::Floor => {
                            map_assets.floor[rng.gen_range(0..map_assets.floor.len())].clone()
                        }
                    };
                    rogue_map
                        .spawn()
                        .insert(Name::new(format!("Tile {}", pt)))
                        .insert(Vector2D::from(pt))
                        .insert(match tile {
                            Tile::Wall => MapTile { is_passable: false },
                            Tile::Floor => MapTile { is_passable: true },
                        })
                        .insert(RenderInfo { texture, z: 0. });
                }
            })
            .id();

        for ipt in info.item_spawns.clone() {
            // TODO: fix item spawning
            // if rng.gen_bool(0.5) {
            //     let damage = rng.gen_range(1..16);
            //     let rate = rng.gen_range(1..16);
            //     let cost = rng.gen_range(4..16);
            //     cmd.spawn()
            //         .insert_bundle(AttackItem::new(
            //             "attack item",
            //             ItemType::MainHand,
            //             AttackBoost::new(damage, rate, cost),
            //             item_assets.skins[rng.gen_range(0..item_assets.skins.len())].clone(),
            //         ))
            //         .insert(Vector2D::from(ipt));
            // } else {
            //     let absorb = rng.gen_range(1..8);
            //     let rate = rng.gen_range(1..8);
            //     let cost = rng.gen_range(4..12);
            //     cmd.spawn()
            //         .insert_bundle(DefenseItem::new(
            //             "defense item",
            //             ItemType::OffHand,
            //             DefenseBoost::new(absorb, rate, cost),
            //             item_assets.skins[rng.gen_range(0..item_assets.skins.len())].clone(),
            //         ))
            //         .insert(Vector2D::from(ipt));
            // }
        }

        let plr_atr = Attributes::new(11, 11, 11, 11, 11, 11);
        // DefaultAttack
        // DefaultDefense
        let team_player = 1;
        cmd.spawn()
            .insert(MovingPlayer {})
            .insert_bundle(Actor::new(
                "Player",
                team_player,
                plr_atr,
                info.player_start,
                player_assets.body.clone(),
                get_player_equipment_slots(),
            ))
            .with_children(|player| {
                // TODO: instead should be equiped inventory
                spawn_player_body_wear(player, &player_assets, options.tile_size);
            });

        team_map[info.player_start] = Some(Team::new(team_player));

        let enemies_id = cmd
            .spawn()
            .insert(Name::new("Enemies"))
            .insert(Transform::default())
            .insert(GlobalTransform::default())
            .with_children(|enms| {
                for mpt in info.monster_spawns.clone() {
                    let mon_atr = Attributes::new(
                        2 + rng.gen_range(0..9),
                        2 + rng.gen_range(0..9),
                        2 + rng.gen_range(0..9),
                        2 + rng.gen_range(0..9),
                        2 + rng.gen_range(0..9),
                        2 + rng.gen_range(0..9),
                    );

                    let team_monster = 1 + rng.gen_range(2..4);

                    enms.spawn()
                        .insert(MovingFovRandom {})
                        .insert_bundle(Actor::new(
                            "Enemy",
                            team_monster,
                            mon_atr,
                            mpt,
                            enemy_assets.skins[rng.gen_range(0..enemy_assets.skins.len())].clone(),
                            vec![(
                                ItemType::MainHand,
                                0,
                                Rect {
                                    top: Val::Px(58.),
                                    left: Val::Px(72.),
                                    ..default()
                                },
                            )],
                        ));

                    team_map[mpt] = Some(Team::new(team_monster));
                }
            })
            .id();

        cmd.insert_resource(map);
        cmd.insert_resource(info);
        cmd.insert_resource(rng);
        cmd.insert_resource(team_map);
        cmd.insert_resource(MapEntities { map_id, enemies_id });

        if let Some(next_state) = state.current().next() {
            state.set(next_state).unwrap();
        } else {
            bevy::log::error!("no next state.");
        }
    }
}

fn spawn_player_body_wear(cb: &mut ChildBuilder, player_assets: &PlayerAssets, size: f32) {
    for i in 0..player_assets.wear.len() {
        cb.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::splat(size)),
                ..Default::default()
            },
            texture: player_assets.wear[i].clone(),
            transform: Transform::from_xyz(0., 0., 4.),
            ..Default::default()
        });
    }
}

fn setup_camera(mut cmd: Commands) {
    cmd.spawn_bundle(OrthographicCameraBundle::new_2d());
    cmd.spawn_bundle(UiCameraBundle::default());
}

pub fn get_player_equipment_slots() -> Vec<(ItemType, u8, Rect<Val>)> {
    vec![
        (
            ItemType::MainHand,
            0,
            Rect {
                top: Val::Px(64. - 16.),
                left: Val::Px(128. - 32. - 16. - 8.),
                ..default()
            },
        ),
        (
            ItemType::OffHand,
            0,
            Rect {
                top: Val::Px(64. - 16.),
                left: Val::Px(128. + 32. - 16. + 8.),
                ..default()
            },
        ),
        (
            ItemType::Head,
            0,
            Rect {
                top: Val::Px(32. - 16. - 8.),
                left: Val::Px(128. - 16.),
                ..default()
            },
        ),
        (
            ItemType::Neck,
            0,
            Rect {
                top: Val::Px(32. - 16. - 8.),
                left: Val::Px(128. + 32. - 16. + 8.),
                ..default()
            },
        ),
        (
            ItemType::Body,
            0,
            Rect {
                top: Val::Px(64. - 16.),
                left: Val::Px(128. - 16.),
                ..default()
            },
        ),
        (
            ItemType::Feet,
            0,
            Rect {
                top: Val::Px(96. - 16. + 8.),
                left: Val::Px(128. - 16.),
                ..default()
            },
        ),
        (
            ItemType::Finger,
            0,
            Rect {
                top: Val::Px(96. - 16. + 8.),
                left: Val::Px(128. - 32. - 16. - 8.),
                ..default()
            },
        ),
        (
            ItemType::Finger,
            1,
            Rect {
                top: Val::Px(96. - 16. + 8.),
                left: Val::Px(128. + 32. - 16. + 8.),
                ..default()
            },
        ),
    ]
}
