pub mod components;
pub mod events;
pub mod map_generator;
pub mod resources;
pub mod systems;

use crate::components::*;
use crate::systems::input;
use crate::systems::moves;
use bevy::log;
use bevy::prelude::*;
use map_generator::{MapGenerator, RandomMapGenerator};
use rand::prelude::*;
use resources::map::Map;
use resources::tile::Tile;
use resources::MapOptions;

pub struct RoguelikePlugin;

impl Plugin for RoguelikePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::create_map);
        app.add_system(input::player_input);
        app.add_system(moves::moves);
        app.register_type::<Vector2D>();
        app.register_type::<Floor>();
        app.register_type::<Wall>();
        log::info!("Loaded Roguelike Plugin");
    }
}

impl RoguelikePlugin {
    pub fn create_map(
        mut commands: Commands,
        map_options: Option<Res<MapOptions>>,
        // window: Res<WindowDescriptor>,
        asset_server: Res<AssetServer>,
    ) {
        let font: Handle<Font> = asset_server.load("fonts/pixeled.ttf");

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
        commands.insert_resource(map.clone());
        commands.insert_resource(info.clone());

        // We define the size of our tiles in world space
        let tile_size = options.tile_size;

        // We deduce the size of the complete board
        let map_size = Vec2::new(
            map.size.x() as f32 * tile_size,
            map.size.y() as f32 * tile_size,
        );
        log::info!("map size: {}", map_size);

        commands
            .spawn()
            .insert(Name::new("RogueMap"))
            .insert(Transform::default())
            .insert(GlobalTransform::default())
            .with_children(|rogue_map| {
                // We spawn the board background sprite at the center of the board, since the sprite pivot is centered
                rogue_map
                    .spawn_bundle(SpriteBundle {
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
            .with_children(|rogue_map| {
                spawn_tiles(rogue_map, &map, tile_size, Color::DARK_GRAY, font.clone());
            });

        let x_offset = map.size.x() as f32 * tile_size / -2.;
        let y_offset = map.size.y() as f32 * tile_size / -2.;

        commands
            .spawn()
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
                    .insert_bundle(get_player_bundle(font.clone(), tile_size));
            });
    }
}

fn spawn_tiles(parent: &mut ChildBuilder, map: &Map, size: f32, color: Color, font: Handle<Font>) {
    let x_offset = map.size.x() as f32 * size / -2.0;
    let y_offset = map.size.y() as f32 * size / -2.0;

    for (pt, tile) in map.enumerate() {
        let mut cmd = parent.spawn();
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
        cmd.with_children(|parent| {
            parent.spawn_bundle(get_tile_bundle(*tile, font.clone(), size));
        });
    }
}

fn get_player_bundle(font: Handle<Font>, size: f32) -> Text2dBundle {
    Text2dBundle {
        text: Text {
            sections: vec![TextSection {
                value: "P".to_string(),
                style: TextStyle {
                    color: Color::RED,
                    font,
                    font_size: size,
                },
            }],
            alignment: TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        },
        transform: Transform::from_xyz(0., 0., 3.),
        ..Default::default()
    }
}

fn get_tile_bundle(tile: Tile, font: Handle<Font>, size: f32) -> Text2dBundle {
    Text2dBundle {
        text: Text {
            sections: vec![TextSection {
                value: match tile {
                    Tile::Wall => "#".to_string(),
                    Tile::Floor => ".".to_string(),
                },
                style: TextStyle {
                    color: match tile {
                        Tile::Wall => Color::GOLD,
                        Tile::Floor => Color::GREEN,
                    },
                    font,
                    font_size: size,
                },
            }],
            alignment: TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        },
        transform: Transform::from_xyz(0., 0., 1.),
        ..Default::default()
    }
}
