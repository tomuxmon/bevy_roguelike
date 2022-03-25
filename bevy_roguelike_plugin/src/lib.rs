pub mod components;
pub mod map_generator;
pub mod resources;

use crate::components::Floor;
use crate::components::Vector2D;
use crate::components::Wall;
use crate::resources::MapPosition;
use bevy::log;
use bevy::prelude::*;
use map_generator::{MapGenerator, RandomMapGenerator};
use rand::prelude::*;
use resources::map::Map;
use resources::tile::Tile;
use resources::MapOptions;
use resources::TileSize;

pub struct RoguelikePlugin;

impl Plugin for RoguelikePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::create_map);
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
        window: Res<WindowDescriptor>,
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

        // We define the size of our tiles in world space
        let tile_size = match options.tile_size {
            TileSize::Fixed(v) => v,
            TileSize::Adaptive { min, max } => adaptative_tile_size(window, min, max, map.size),
        };

        // We deduce the size of the complete board
        let map_size = Vec2::new(
            map.size.x() as f32 * tile_size,
            map.size.y() as f32 * tile_size,
        );
        log::info!("map size: {}", map_size);
        // We define the board anchor position (bottom left)
        let map_position = match options.position {
            MapPosition::Centered { offset } => {
                Vec3::new(-(map_size.x / 2.), -(map_size.y / 2.), 0.) + offset
            }
            MapPosition::Custom(p) => p,
        };

        commands
            .spawn()
            .insert(Name::new("RogueMap"))
            .insert(Transform::from_translation(map_position))
            .insert(GlobalTransform::default())
            .with_children(|parent| {
                // We spawn the board background sprite at the center of the board, since the sprite pivot is centered
                parent
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::BLACK,
                            custom_size: Some(map_size),
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(map_size.x / 2., map_size.y / 2., 0.),
                        ..Default::default()
                    })
                    .insert(Name::new("Background")); // We probably do not need that...
            })
            .with_children(|parent| {
                spawn_tiles(parent, &map, tile_size, Color::DARK_GRAY, font);
            });
    }
}

fn spawn_tiles(parent: &mut ChildBuilder, map: &Map, size: f32, color: Color, font: Handle<Font>) {
    for (pt, tile) in map.enumerate() {
        let mut cmd = parent.spawn();
        cmd.insert_bundle(SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::splat(size)),
                ..Default::default()
            },
            transform: Transform::from_xyz(
                (pt.x() as f32 * size) + (size / 2.),
                (pt.y() as f32 * size) + (size / 2.),
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

        // cmd.with_children(|parent| {
        //     parent.spawn_bundle(SpriteBundle {
        //         sprite: Sprite {
        //             custom_size: Some(Vec2::splat(size)),
        //             ..Default::default()
        //         },
        //         transform: Transform::from_xyz(0., 0., 1.),
        //         // texture: bomb_image.clone(),
        //         ..Default::default()
        //     });
        // });
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

/// Computes a tile size that matches the window according to the tile map size
fn adaptative_tile_size(
    window: Res<WindowDescriptor>,
    min: f32,       // Tile size constraint
    max: f32,       // Tile size constraint
    size: Vector2D, // Tile map dimensions
) -> f32 {
    let max_width = window.width / size.x() as f32;
    let max_heigth = window.height / size.y() as f32;
    max_width.min(max_heigth).clamp(min, max)
}
