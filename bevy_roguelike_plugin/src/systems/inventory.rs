use crate::{components::*, dragable_ui::DragableUI, events::*, resources::*};
use bevy::{prelude::*, ui::*};

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

pub fn get_gear_image_bundle(
    tile_size: f32,
    top_px: f32,
    right_px: f32,
    image: UiImage,
) -> ImageBundle {
    ImageBundle {
        style: Style {
            size: Size::new(Val::Px(tile_size), Val::Px(tile_size)),
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(top_px),
                right: Val::Px(right_px),
                ..default()
            },
            ..default()
        },
        image,
        ..Default::default()
    }
}

pub fn toggle_inventory_open(
    mut cmd: Commands,
    keys: Res<Input<KeyCode>>,
    inventory_assets: Res<InventoryAssets>,
    map_options: Res<MapOptions>,
    inventory_display: Query<Entity, With<InventoryDisplay>>,
) {
    if !keys.just_pressed(KeyCode::I) {
        return;
    }
    if let Ok(inv) = inventory_display.get_single() {
        cmd.entity(inv).despawn_recursive();
    } else {
        cmd.spawn()
            .insert(Name::new("inventory display"))
            .insert(InventoryDisplay {})
            .insert(DragableUI::default())
            .insert(Interaction::default())
            .insert_bundle(NodeBundle {
                style: Style {
                    flex_wrap: FlexWrap::Wrap,
                    flex_direction: FlexDirection::ColumnReverse,
                    size: Size::new(Val::Px(256.0), Val::Auto),
                    position_type: PositionType::Absolute,
                    position: Rect {
                        top: Val::Px(10.0),
                        right: Val::Px(10.0),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                // NOTE: drag area and player gear
                parent
                    .spawn()
                    .insert(Name::new("drag and gear"))
                    .insert_bundle(NodeBundle {
                        focus_policy: FocusPolicy::Pass,
                        style: Style {
                            size: Size::new(Val::Px(256.0), Val::Px(128.)),
                            ..default()
                        },
                        // TODO: should be an image instead of a color
                        color: Color::rgba(0.0125, 0.05, 0.025, 0.9).into(),
                        ..default()
                    })
                    .with_children(|cb| {
                        cb.spawn().insert(Name::new("body wear")).insert_bundle(
                            get_gear_image_bundle(
                                map_options.tile_size,
                                64. - 16.,
                                128. - 16.,
                                inventory_assets.slot_body_wear.clone().into(),
                            ),
                        );
                        cb.spawn().insert(Name::new("head wear")).insert_bundle(
                            get_gear_image_bundle(
                                map_options.tile_size,
                                32. - 16. - 8.,
                                128. - 16.,
                                inventory_assets.slot_head_wear.clone().into(),
                            ),
                        );
                        cb.spawn().insert(Name::new("feet wear")).insert_bundle(
                            get_gear_image_bundle(
                                map_options.tile_size,
                                96. - 16. + 8.,
                                128. - 16.,
                                inventory_assets.slot_feet_wear.clone().into(),
                            ),
                        );
                        cb.spawn()
                            .insert(Name::new("main hand gear"))
                            .insert_bundle(get_gear_image_bundle(
                                map_options.tile_size,
                                64. - 16.,
                                160. - 16. + 8.,
                                inventory_assets.slot_main_hand_gear.clone().into(),
                            ));
                        cb.spawn().insert(Name::new("finger wear")).insert_bundle(
                            get_gear_image_bundle(
                                map_options.tile_size,
                                96. - 16. + 8.,
                                160. - 16. + 8.,
                                inventory_assets.slot_finger_wear.clone().into(),
                            ),
                        );
                        cb.spawn().insert(Name::new("neck wear")).insert_bundle(
                            get_gear_image_bundle(
                                map_options.tile_size,
                                32. - 16. - 8.,
                                96. - 16. - 8.,
                                inventory_assets.slot_neck_wear.clone().into(),
                            ),
                        );
                        cb.spawn().insert(Name::new("off hand gear")).insert_bundle(
                            get_gear_image_bundle(
                                map_options.tile_size,
                                64. - 16.,
                                96. - 16. - 8.,
                                inventory_assets.slot_off_hand_gear.clone().into(),
                            ),
                        );
                        cb.spawn().insert(Name::new("finger wear")).insert_bundle(
                            get_gear_image_bundle(
                                map_options.tile_size,
                                96. - 16. + 8.,
                                96. - 16. - 8.,
                                inventory_assets.slot_finger_wear.clone().into(),
                            ),
                        );
                    });
                // NOTE: spawn inventory with slots
                parent
                    .spawn()
                    .insert(Name::new("inventory"))
                    .insert_bundle(NodeBundle {
                        style: Style {
                            flex_wrap: FlexWrap::WrapReverse,
                            flex_direction: FlexDirection::Row,
                            size: Size::new(Val::Px(256.0), Val::Auto),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|cb| {
                        for i in 0..Inventory::DEFAULT_CAPACITY {
                            cb.spawn()
                                .insert(Name::new(format!("Slot {}", i)))
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
                        }
                    });
            });
    }
}

pub fn inventory_update(
    mut cmd: Commands,
    map_options: Res<MapOptions>,
    player_inventory: Query<&Inventory, With<MovingPlayer>>,
    items: Query<(&RenderInfo, Option<&Equiped>), With<Item>>,
    item_slots: Query<(Entity, &ItemCarySlot)>,
) {
    let inventory = if let Ok(i) = player_inventory.get_single() {
        i
    } else {
        return;
    };
    for (ee, slot) in item_slots.iter() {
        if let Some(item) = inventory[slot.index()] {
            if let Ok((info, _eqiuped)) = items.get(item) {
                // TODO: check if item already in the slot
                // no need to rerender it

                cmd.entity(ee).with_children(|cb| {
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
}
