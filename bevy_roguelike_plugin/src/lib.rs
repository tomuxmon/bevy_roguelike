pub mod components;
pub mod events;
pub mod map_generator;
pub mod resources;
pub mod systems;

use crate::components::*;
use crate::events::MoveEvent;
use bevy::ecs::schedule::StateData;
use bevy::log;
use bevy::prelude::*;
use map_generator::{MapGenerator, RandomMapGenerator};
use rand::prelude::*;
use resources::map::Map;
use resources::map_assets::MapAssets;
use resources::player_assets::PlayerAssets;
use resources::tile::Tile;
use resources::MapOptions;

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
                .with_system(systems::input::player_input)
                .with_system(systems::moves::moves),
        )
        .add_system_set(
            SystemSet::on_exit(self.running_state.clone()).with_system(Self::cleanup_map),
        )
        .register_type::<Vector2D>()
        .register_type::<Floor>()
        .register_type::<Wall>()
        .add_event::<MoveEvent>();
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

        // We define the size of our tiles in world space
        let tile_size = options.tile_size;

        // We deduce the size of the complete board
        let map_size = Vec2::new(
            map.size.x() as f32 * tile_size,
            map.size.y() as f32 * tile_size,
        );
        log::info!("map size: {}", map_size);

        let map_id = cmd
            .spawn()
            .insert(Name::new("RogueMap"))
            .insert(Transform::default())
            .insert(GlobalTransform::default())
            .with_children(|cb| {
                // We spawn the board background sprite at the center of the board, since the sprite pivot is centered
                cb.spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::BLACK,
                        custom_size: Some(map_size),
                        ..Default::default()
                    },
                    transform: Transform::default(),
                    ..Default::default()
                })
                .insert(Name::new("Background")); // We probably do not need that...
            })
            .with_children(|cb| {
                spawn_tiles(
                    cb,
                    &map_assets,
                    &map,
                    &mut rng,
                    tile_size,
                    Color::DARK_GRAY,
                    // font.clone(),
                );
            })
            .id();

        let x_offset = map.size.x() as f32 * tile_size / -2.;
        let y_offset = map.size.y() as f32 * tile_size / -2.;

        cmd.spawn()
            .insert(Name::new("Player"))
            .insert(Player {})
            .insert(info.player_start)
            .insert(Transform::from_xyz(
                (info.player_start.x() as f32 * tile_size) + (tile_size / 2.) + x_offset,
                (info.player_start.y() as f32 * tile_size) + (tile_size / 2.) + y_offset,
                2.,
            ))
            .insert(GlobalTransform::default())
            .with_children(|player| {
                player
                    .spawn()
                    .insert(Name::new("Player looks"))
                    .insert_bundle(get_player_bundle(&player_assets, tile_size));
            });

        cmd.insert_resource(MapId { id: map_id });
    }
}

fn spawn_tiles(
    cb: &mut ChildBuilder,
    map_assets: &MapAssets,
    map: &Map,
    rng: &mut StdRng,
    size: f32,
    color: Color,
) {
    let x_offset = map.size.x() as f32 * size / -2.0;
    let y_offset = map.size.y() as f32 * size / -2.0;

    for (pt, tile) in map.enumerate() {
        let mut cmd = cb.spawn();
        cmd.insert_bundle(SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::splat(size)),
                ..Default::default()
            },
            transform: Transform::from_xyz(
                (pt.x() as f32 * size) + (size / 2.) + x_offset,
                (pt.y() as f32 * size) + (size / 2.) + y_offset,
                1.,
            ),
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
            tile_cb.spawn_bundle(get_tile_bundle(*tile, map_assets, rng, size));
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

    // Text2dBundle {
    //     text: Text {
    //         sections: vec![TextSection {
    //             value: "P".to_string(),
    //             style: TextStyle {
    //                 color: Color::RED,
    //                 font,
    //                 font_size: size,
    //             },
    //         }],
    //         alignment: TextAlignment {
    //             vertical: VerticalAlign::Center,
    //             horizontal: HorizontalAlign::Center,
    //         },
    //     },
    //     transform: Transform::from_xyz(0., 0., 3.),
    //     ..Default::default()
    // }
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

    // Text2dBundle {
    //     text: Text {
    //         sections: vec![TextSection {
    //             value: match tile {
    //                 Tile::Wall => "#".to_string(),
    //                 Tile::Floor => ".".to_string(),
    //             },
    //             style: TextStyle {
    //                 color: match tile {
    //                     Tile::Wall => Color::GOLD,
    //                     Tile::Floor => Color::GREEN,
    //                 },
    //                 font,
    //                 font_size: size,
    //             },
    //         }],
    //         alignment: TextAlignment {
    //             vertical: VerticalAlign::Center,
    //             horizontal: HorizontalAlign::Center,
    //         },
    //     },
    //     transform: Transform::from_xyz(0., 0., 1.),
    //     ..Default::default()
    // }
}
