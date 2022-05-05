use crate::{components::*, resources::MapOptions};
use bevy::prelude::*;

pub fn render_body(
    mut cmd: Commands,
    renderables: Query<(Entity, &Vector2D, &RenderInfo), Without<VisibilityToggle>>,
    map_options: Res<MapOptions>,
) {
    for (rendity, pt, info) in renderables.iter() {
        cmd.entity(rendity)
            .insert(VisibilityToggle::default())
            .insert(Transform::from_translation(
                map_options.to_world_position(**pt).extend(info.z),
            ))
            .insert(GlobalTransform::default())
            .with_children(|renderable| {
                renderable
                    .spawn()
                    .insert(Name::new("body"))
                    .insert_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::WHITE,
                            custom_size: Some(Vec2::splat(map_options.tile_size)),
                            ..Default::default()
                        },
                        texture: info.texture.clone(),
                        transform: Transform::from_xyz(0., 0., info.z + 0.1),
                        ..Default::default()
                    });
            });
    }
}

pub fn render_hud_health_bar(
    mut cmd: Commands,
    renderables: Query<Entity, (With<HitPoints>, Without<HudHealthBar>)>,
    map_options: Res<MapOptions>,
) {
    for rendity in renderables.iter() {
        cmd.entity(rendity)
            .insert(HudHealthBar {})
            .with_children(|renderable| {
                let height = map_options.tile_size / 16.;
                renderable
                    .spawn()
                    .insert(Name::new("hud"))
                    .insert(HudHealthBar {})
                    .insert_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::GREEN,
                            custom_size: Some(Vec2::new(map_options.tile_size, height)),
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(
                            0.,
                            -map_options.tile_size / 2. + height / 2.,
                            100.,
                        ),
                        ..Default::default()
                    });
            });
    }
}

// TODO: render inventory 
