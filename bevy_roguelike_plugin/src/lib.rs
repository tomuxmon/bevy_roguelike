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
use map_generator::{MapGenerator, RandomMapGenerator};
use rand::prelude::*;
use resources::*;

pub struct RoguelikePlugin<T> {
    pub running_state: T,
}

impl<T: StateData> Plugin for RoguelikePlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(self.running_state.clone()).with_system(Self::create_map),
        )
        .add_system_set(
            SystemSet::on_update(self.running_state.clone())
                .with_system(systems::turns::apply_hp_modify)
                .with_system(systems::turns::gather_action_points)
                .with_system(systems::turns::turn_end_now_gather_or_die)
                .with_system(systems::camera::camera_set_focus_player)
                .with_system(systems::camera::camera_focus_immediate),
        )
        .add_system_set(
            SystemSet::on_exit(self.running_state.clone()).with_system(Self::cleanup_map),
        )
        .register_type::<Vector2D>()
        .register_type::<Floor>()
        .register_type::<Wall>()
        .register_type::<Behaviour>()
        .register_type::<ActionPoints>()
        .register_type::<HitPoints>()
        .register_type::<TurnState>()
        .register_type::<Team>()
        .add_event::<ModifyHPEvent>();

        log::info!("Loaded Roguelike Plugin");
    }
}

#[derive(Debug)]
pub struct MapId {
    id: Entity,
}

impl<T> RoguelikePlugin<T> {
    fn cleanup_map(map_id: Res<MapId>, mut cmd: Commands) {
        cmd.entity(map_id.id).despawn_recursive();
        cmd.remove_resource::<MapId>();
    }

    pub fn create_map(
        mut cmd: Commands,
        map_options: Option<Res<MapOptions>>,
        map_assets: Res<MapAssets>,
        player_assets: Res<PlayerAssets>,
        enemy_assets: Res<EnemyAssets>,
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
        let (map, info) = map_generator.gen(&mut rng, options.map_size);
        log::info!("{}", map.to_colorized_string());
        log::info!("{}", info.to_colorized_string());
        cmd.insert_resource(map.clone());
        cmd.insert_resource(info.clone());
        cmd.insert_resource(rng.clone());

        let map_id = cmd
            .spawn()
            .insert(Name::new("RogueMap"))
            .insert(Transform::default())
            .insert(GlobalTransform::default())
            .with_children(|cb| {
                cb.spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::BLACK,
                        custom_size: Some(Vec2::new(
                            map.size.x() as f32 * options.tile_size,
                            map.size.y() as f32 * options.tile_size,
                        )),
                        ..Default::default()
                    },
                    transform: Transform::default(),
                    ..Default::default()
                })
                .insert(Name::new("Background"));
            })
            .with_children(|cb| {
                spawn_tiles(cb, &map_assets, &options, &map, &mut rng, Color::DARK_GRAY);
            })
            .id();

        let increment_default = 100;

        cmd.spawn()
            .insert(Name::new("Player"))
            .insert(Player {})
            .insert(Behaviour::InputControlled)
            .insert(Team::new(1))
            .insert(TurnState::default())
            .insert(ActionPoints::new(increment_default + rng.gen_range(0..128)))
            .insert(HitPoints::new(
                HitPoints::DEFAULT_MAX + rng.gen_range(0..128),
            ))
            .insert(info.player_start)
            .insert(Transform::from_translation(
                options.to_world_position(info.player_start).extend(2.),
            ))
            .insert(GlobalTransform::default())
            .with_children(|player| {
                player
                    .spawn()
                    .insert(Name::new("player body"))
                    .insert_bundle(get_player_bundle(&player_assets, options.tile_size))
                    .with_children(|body_cb| {
                        spawn_player_wear(body_cb, &player_assets, options.tile_size)
                    });
            });

        // TODO: spawn enemies

        cmd.spawn()
            .insert(Name::new("Enemies"))
            .insert(Transform::default())
            .insert(GlobalTransform::default())
            .with_children(|enms| {
                for mpt in info.monster_spawns {
                    enms.spawn()
                        .insert(Name::new("Enemy"))
                        .insert(Enemy {})
                        .insert(Team::new(1 + rng.gen_range(1..4)))
                        .insert(TurnState::default())
                        .insert(ActionPoints::new(increment_default + rng.gen_range(0..128)))
                        .insert(HitPoints::new(
                            HitPoints::DEFAULT_MAX + rng.gen_range(0..128),
                        ))
                        .insert(Behaviour::RandomMove)
                        .insert(mpt)
                        .insert(Transform::from_translation(
                            options.to_world_position(mpt).extend(2.),
                        ))
                        .insert(GlobalTransform::default())
                        .with_children(|enemy| {
                            enemy.spawn().insert(Name::new("enemy body")).insert_bundle(
                                get_enemy_bundle(&enemy_assets, &mut rng, options.tile_size),
                            );
                        });
                }
            });

        cmd.insert_resource(MapId { id: map_id });
    }
}

fn spawn_tiles(
    cb: &mut ChildBuilder,
    map_assets: &MapAssets,
    map_options: &MapOptions,
    map: &Map,
    rng: &mut StdRng,
    color: Color,
) {
    for (pt, tile) in map.enumerate() {
        let mut cmd = cb.spawn();
        cmd.insert_bundle(SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::splat(map_options.tile_size)),
                ..Default::default()
            },
            transform: Transform::from_translation(map_options.to_world_position(pt).extend(1.)),
            ..Default::default()
        })
        .insert(Name::new(format!("Tile {}", pt)))
        .insert(pt);

        match tile {
            Tile::Wall => {
                cmd.insert(Wall {});
            }
            Tile::Floor => {
                cmd.insert(Floor {});
            }
        }
        cmd.with_children(|tile_cb| {
            tile_cb.spawn_bundle(get_tile_bundle(
                *tile,
                map_assets,
                rng,
                map_options.tile_size,
            ));
        });
    }
}

fn get_player_bundle(player_assets: &PlayerAssets, size: f32) -> impl Bundle {
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
fn spawn_player_wear(cb: &mut ChildBuilder, player_assets: &PlayerAssets, size: f32) {
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

fn get_enemy_bundle(enemy_assets: &EnemyAssets, rng: &mut StdRng, size: f32) -> impl Bundle {
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

fn get_tile_bundle(tile: Tile, map_assets: &MapAssets, rng: &mut StdRng, size: f32) -> impl Bundle {
    let texture = match tile {
        Tile::Wall => map_assets.wall[rng.gen_range(0..map_assets.wall.len())].clone(),
        Tile::Floor => map_assets.floor[rng.gen_range(0..map_assets.floor.len())].clone(),
    };
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
