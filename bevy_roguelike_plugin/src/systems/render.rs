use crate::{components::*, resources::MapOptions};
use bevy::prelude::*;
use bevy_inventory::{Equipment, ItemType};
use bevy_roguelike_combat::HitPoints;

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
                    .insert(Name::new("render"))
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

                for cosmetic_texture in info.cosmetic_textures.iter() {
                    renderable
                        .spawn()
                        .insert(Name::new("render cosmetic"))
                        .insert_bundle(SpriteBundle {
                            sprite: Sprite {
                                color: Color::WHITE,
                                custom_size: Some(Vec2::splat(map_options.tile_size)),
                                ..Default::default()
                            },
                            texture: cosmetic_texture.clone(),
                            transform: Transform::from_xyz(0., 0., info.z + 0.2),
                            ..Default::default()
                        });
                }
            });
    }
}

#[allow(clippy::type_complexity)]
pub fn render_equiped_item<I: ItemType>(
    mut cmd: Commands,
    actors: Query<(Entity, &Vector2D), With<Equipment<I>>>,
    items: Query<
        (Entity, &RenderInfoEquiped, &ItemEquipedOwned),
        (With<I>, Without<EquipedRendition>),
    >,
    map_options: Res<MapOptions>,
) {
    for (item_entity, info, owner) in items.iter() {
        if let Ok((_, _pt)) = actors.get(owner.actor) {
            let mut some_item_render_entity = None;
            cmd.entity(owner.actor).with_children(|renderable| {
                let item_render_entity = renderable
                    .spawn()
                    .insert(Name::new("item render"))
                    .insert(EquipedRenderedItem { item: item_entity })
                    .insert_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::WHITE,
                            custom_size: Some(Vec2::splat(map_options.tile_size)),
                            ..Default::default()
                        },
                        texture: info.texture.clone(),
                        transform: Transform::from_xyz(0., 0., info.z + 0.1),
                        ..Default::default()
                    })
                    .id();
                some_item_render_entity = Some(item_render_entity);
            });
            if let Some(item_render_entity) = some_item_render_entity {
                cmd.entity(item_entity)
                    .insert(EquipedRendition { item_render_entity });
            }
        }
    }
}
pub fn unrender_unequiped_items(
    mut cmd: Commands,
    items: Query<(Entity, &EquipedRendition), Without<ItemEquipedOwned>>,
    renditions: Query<&EquipedRenderedItem>,
) {
    for (item_entity, rendition) in items.iter() {
        cmd.entity(item_entity).remove::<EquipedRendition>();
        if renditions.get(rendition.item_render_entity).is_ok() {
            cmd.entity(rendition.item_render_entity).despawn_recursive();
        } else {
            // TODO: close inventory when actor is dead;
            bevy::log::info!("item render entity not found. could not despawn.");
        }
    }
}

pub fn render_hud_health_bar(
    mut cmd: Commands,
    renderables: Query<Entity, (With<HitPoints<RogueAttributeType>>, Without<HudHealthBar>)>,
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
