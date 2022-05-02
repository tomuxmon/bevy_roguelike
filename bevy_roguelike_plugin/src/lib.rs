pub mod components;
pub mod events;
pub mod map_generator;
pub mod resources;
pub mod systems;

use crate::components::*;
use crate::events::*;
use bevy::ecs::schedule::StateData;
use bevy::log;
use bevy::prelude::*;
use bevy_easings::*;
use map_generator::*;
use rand::prelude::*;
use resources::*;
use systems::actor_stats::*;
use systems::camera::*;
use systems::fov::*;
use systems::turns::*;

pub struct RoguelikePlugin<T> {
    pub running_state: T,
}

impl<T: StateData> Plugin for RoguelikePlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_plugin(EasingsPlugin {})
            .add_startup_system(camera_setup)
            .add_system_set(
                SystemSet::on_enter(self.running_state.clone()).with_system(Self::create_map),
            )
            .add_system_set(
                SystemSet::on_update(self.running_state.clone())
                    .with_system(attributes_update_action_points)
                    .with_system(attributes_update_hit_points)
                    .with_system(attributes_update_attack_stats)
                    .with_system(attributes_update_field_of_view)
                    .with_system(gather_action_points)
                    .with_system(turn_end_now_gather.after(gather_action_points))
                    .with_system(act)
                    .with_system(spend_ap.after(act))
                    .with_system(do_move.after(act).after(spend_ap))
                    .with_system(apply_position_to_transform.after(do_move))
                    .with_system(apply_hp_modify.after(act).after(spend_ap))
                    .with_system(idle_rest.after(apply_hp_modify))
                    .with_system(camera_set_focus_player)
                    .with_system(camera_focus_smooth.after(camera_set_focus_player))
                    .with_system(field_of_view_recompute)
                    .with_system(field_of_view_set_vis_info.after(field_of_view_recompute))
                    .with_system(field_of_view_set_vis.after(field_of_view_set_vis_info)),
            )
            .add_system_set(
                SystemSet::on_exit(self.running_state.clone()).with_system(Self::cleanup_map),
            )
            .register_type::<Vector2D>()
            .register_type::<MapTile>()
            .register_type::<Attributes>()
            .register_type::<ActionPoints>()
            .register_type::<AttackStats>()
            .register_type::<HitPoints>()
            .register_type::<TurnState>()
            .register_type::<ModifyHP>()
            .register_type::<Team>()
            .register_type::<MovingPlayer>()
            .register_type::<MovingRandom>()
            .register_type::<MovingFovRandom>()
            .register_type::<FieldOfView>()
            .add_event::<SpendAPEvent>()
            .add_event::<MoveEvent>()
            .add_event::<ActEvent>()
            .add_event::<IdleEvent>()
            .add_event::<CameraFocusEvent>();

        log::info!("Loaded Roguelike Plugin");
    }
}

#[derive(Debug)]
pub struct MapEntities {
    map_id: Entity,
    enemies_id: Entity,
}

impl<T> RoguelikePlugin<T> {
    fn cleanup_map(map_id: Res<MapEntities>, mut cmd: Commands) {
        cmd.entity(map_id.map_id).despawn_recursive();
        cmd.entity(map_id.enemies_id).despawn_recursive();
        cmd.remove_resource::<MapEntities>();
    }

    pub fn create_map(
        mut cmd: Commands,
        map_options: Option<Res<MapOptions>>,
        map_assets: Res<MapAssets>,
        player_assets: Res<PlayerAssets>,
        enemy_assets: Res<EnemyAssets>,
        item_assets: Res<ItemAssets>,
        mut cameras: Query<&mut Transform, With<Camera>>,
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

        let map_id = cmd
            .spawn()
            .insert(Name::new("RogueMap"))
            .insert(Transform::default())
            .insert(GlobalTransform::default())
            .with_children(|rogue_map| {
                spawn_tiles(rogue_map, &map_assets, &options, &map, &mut rng);
            })
            .id();

        // TODO: spawn few items

        for ipt in info.item_spawns.clone() {
            cmd.spawn()
                .insert(Name::new("Item"))
                .insert(Item {})
                .insert(VisibilityToggle::default())
                .insert(Vector2D::from(ipt))
                .insert(Transform::from_translation(
                    options.to_world_position(ipt).extend(1.),
                ))
                .insert(GlobalTransform::default())
                .with_children(|item| {
                    item.spawn().insert_bundle(get_item_body_bundle(
                        &item_assets,
                        &mut rng,
                        options.tile_size,
                    ));
                });
        }

        let plr_atr = Attributes::new(11, 11, 11, 11, 11, 11);
        let team_player = 1;
        cmd.spawn()
            .insert(MovingPlayer {})
            .insert_bundle(Actor::new(
                "Player",
                team_player,
                plr_atr,
                info.player_start,
                &options,
            ))
            .with_children(|player| {
                player
                    .spawn()
                    .insert(Name::new("body"))
                    .insert_bundle(get_player_body_bundle(&player_assets, options.tile_size));

                spawn_player_body_wear(player, &player_assets, options.tile_size);

                player
                    .spawn()
                    .insert(Name::new("hud"))
                    .insert(OnTopHud {})
                    .insert_bundle(get_hud_bundle(options.tile_size));
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

                    let team_monster = 1 + rng.gen_range(1..4);

                    enms.spawn()
                        .insert(MovingFovRandom {})
                        .insert_bundle(Actor::new("Enemy", team_monster, mon_atr, mpt, &options))
                        .with_children(|enemy| {
                            enemy.spawn().insert(Name::new("body")).insert_bundle(
                                get_enemy_body_bundle(&enemy_assets, &mut rng, options.tile_size),
                            );
                            enemy
                                .spawn()
                                .insert(Name::new("hud"))
                                .insert(OnTopHud {})
                                .insert_bundle(get_hud_bundle(options.tile_size));
                        });

                    team_map[mpt] = Some(Team::new(team_monster));
                }
            })
            .id();

        cmd.insert_resource(map);
        cmd.insert_resource(info);
        cmd.insert_resource(rng);
        cmd.insert_resource(team_map);
        cmd.insert_resource(MapEntities { map_id, enemies_id });
    }
}

fn spawn_tiles(
    cb: &mut ChildBuilder,
    map_assets: &MapAssets,
    map_options: &MapOptions,
    map: &Map,
    rng: &mut StdRng,
) {
    for (pt, tile) in map.enumerate() {
        let texture = match tile {
            Tile::Wall => map_assets.wall[rng.gen_range(0..map_assets.wall.len())].clone(),
            Tile::Floor => map_assets.floor[rng.gen_range(0..map_assets.floor.len())].clone(),
        };
        cb.spawn()
            .insert(Name::new(format!("Tile {}", pt)))
            .insert(Transform::from_translation(
                map_options.to_world_position(pt).extend(1.),
            ))
            .insert(GlobalTransform::default())
            .insert(VisibilityToggle::default())
            .insert(Vector2D::from(pt))
            .insert(match tile {
                Tile::Wall => MapTile { is_passable: false },
                Tile::Floor => MapTile { is_passable: true },
            })
            .with_children(|cb| {
                cb.spawn()
                    .insert(Name::new("body"))
                    .insert_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::WHITE,
                            custom_size: Some(Vec2::splat(map_options.tile_size)),
                            ..Default::default()
                        },
                        texture,
                        ..Default::default()
                    });
            });
    }
}

fn get_hud_bundle(size: f32) -> impl Bundle {
    let height = size / 16.;
    SpriteBundle {
        sprite: Sprite {
            color: Color::GREEN,
            custom_size: Some(Vec2::new(size, height)),
            ..Default::default()
        },
        transform: Transform::from_xyz(0., -size / 2. + height / 2., 100.),
        ..Default::default()
    }
}

fn get_player_body_bundle(player_assets: &PlayerAssets, size: f32) -> impl Bundle {
    SpriteBundle {
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::splat(size)),
            ..Default::default()
        },
        texture: player_assets.body.clone(),
        transform: Transform::from_xyz(0., 0., 3.),
        ..Default::default()
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

fn get_enemy_body_bundle(enemy_assets: &EnemyAssets, rng: &mut StdRng, size: f32) -> impl Bundle {
    let texture = enemy_assets.skins[rng.gen_range(0..enemy_assets.skins.len())].clone();
    SpriteBundle {
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::splat(size)),
            ..Default::default()
        },
        texture,
        transform: Transform::from_xyz(0., 0., 3.),
        ..Default::default()
    }
}

fn get_item_body_bundle(item_assets: &ItemAssets, rng: &mut StdRng, size: f32) -> impl Bundle {
    let texture = item_assets.skins[rng.gen_range(0..item_assets.skins.len())].clone();
    SpriteBundle {
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::splat(size)),
            ..Default::default()
        },
        texture,
        transform: Transform::from_xyz(0., 0., 1.),
        ..Default::default()
    }
}

fn camera_setup(mut cmd: Commands) {
    cmd.spawn_bundle(OrthographicCameraBundle::new_2d());
}
