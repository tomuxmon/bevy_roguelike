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
                        sprite: info.sprite.clone(),
                        texture: info.texture.clone(),
                        transform: Transform::from_xyz(0., 0., info.z + 0.1),
                        ..Default::default()
                    });
            });
    }
}
