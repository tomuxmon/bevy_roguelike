use crate::{components::*, events::*, resources::*};
use bevy::prelude::*;

pub fn pick_up_items(
    mut cmd: Commands,
    mut pick_up_item_reader: EventReader<PickUpItemEvent>,
    mut actors: Query<(&Vector2D, &mut Inventory)>,
    items: Query<
        (Entity, &Vector2D, &Children),
        (
            With<Item>,
            With<Transform>,
            With<GlobalTransform>,
            With<VisibilityToggle>,
        ),
    >,
) {
    for e in pick_up_item_reader.iter() {
        if let Ok((actor_pt, mut inventory)) = actors.get_mut(e.picker) {
            for (item_entity, _, children) in items.iter().filter(|(_, pt, _)| **pt == *actor_pt) {
                if inventory.add(item_entity) {
                    for c in children.iter() {
                        cmd.entity(*c).despawn_recursive();
                    }
                    cmd.entity(item_entity)
                        .remove::<Vector2D>()
                        .remove::<Transform>()
                        .remove::<GlobalTransform>()
                        .remove::<VisibilityToggle>()
                        // TODO: remove when inventory handling implemented
                        .insert(Equiped {});
                }
            }
        }
    }
}

pub fn drop_item(
    mut cmd: Commands,
    mut drop_reader: EventReader<DropItemEvent>,
    mut actors: Query<(&Vector2D, &mut Inventory)>,
) {
    for e in drop_reader.iter() {
        if let Ok((pt, mut inventory)) = actors.get_mut(e.droper) {
            if let Some(_) = inventory.take(e.item) {
                cmd.entity(e.item).insert(*pt);
            }
        }
    }
}

pub fn toggle_inventory_open(
    mut cmd: Commands,
    keys: Res<Input<KeyCode>>,
    inventory_assets: Res<InventoryAssets>,
    map_options: Res<MapOptions>,
    inventory_display: Query<Entity, With<InventoryDisplay>>,
    player_inventory: Query<&Inventory, With<MovingPlayer>>,
    items: Query<(&RenderInfo, Option<&Equiped>), With<Item>>,
) {
    if !keys.just_pressed(KeyCode::I) {
        return;
    }
    let inventory = if let Ok(i) = player_inventory.get_single() {
        i
    } else {
        return;
    };
    if let Ok(inv) = inventory_display.get_single() {
        cmd.entity(inv).despawn_recursive();
    } else {
        cmd.spawn()
            .insert(Name::new("inventory"))
            .insert(InventoryDisplay {})
            .insert_bundle(NodeBundle {
                style: Style {
                    flex_wrap: FlexWrap::WrapReverse,
                    flex_direction: FlexDirection::Row,
                    align_content: AlignContent::FlexStart,
                    size: Size::new(Val::Px(200.0), Val::Px(170.0)),
                    position_type: PositionType::Absolute,
                    position: Rect {
                        top: Val::Px(20.0),
                        right: Val::Px(20.0),
                        ..default()
                    },
                    border: Rect {
                        left: Val::Px(4.0),
                        right: Val::Px(4.0),
                        top: Val::Px(16.0),
                        bottom: Val::Px(4.0),
                    },
                    ..default()
                },
                color: Color::rgba(0.09, 0.11, 0.1, 0.9).into(),
                ..default()
            })
            .with_children(|parent| {
                for i in 0..Inventory::DEFAULT_CAPACITY {
                    let mut pcmd = parent.spawn();
                    pcmd.insert(Name::new(format!("Slot {}", i)))
                        .insert(ItemCarySlot::new(i))
                        .insert_bundle(ImageBundle {
                            style: Style {
                                size: Size::new(
                                    Val::Px(map_options.tile_size),
                                    Val::Px(map_options.tile_size),
                                ),
                                ..default()
                            },
                            image: inventory_assets.slot.clone().into(),
                            ..Default::default()
                        });

                    if let Some(item) = inventory[i] {
                        if let Ok((info, _eqiuped)) = items.get(item) {
                            // if eqiuped.is_some() {
                            //     pcmd.with_children(|cb| {
                            //         cb.spawn().insert_bundle(ImageBundle {
                            //             style: Style {
                            //                 size: Size::new(
                            //                     Val::Px(map_options.tile_size),
                            //                     Val::Px(map_options.tile_size),
                            //                 ),
                            //                 ..default()
                            //             },
                            //             image: inventory_assets.slot.clone().into(),
                            //             ..Default::default()
                            //         });
                            //     });
                            // }
                            pcmd.with_children(|cb| {
                                cb.spawn().insert_bundle(ImageBundle {
                                    style: Style {
                                        size: Size::new(
                                            Val::Px(map_options.tile_size),
                                            Val::Px(map_options.tile_size),
                                        ),
                                        ..default()
                                    },
                                    image: info.texture.clone().into(),
                                    ..Default::default()
                                });
                            });
                        } else {
                            bevy::log::error!("item in inventory but not in the world.");
                        }
                    }
                }
            });
    }
}
