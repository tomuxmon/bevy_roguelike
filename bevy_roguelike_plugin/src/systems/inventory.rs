use crate::{components::*, dragable_ui::DragableUI, events::*, resources::*};
use bevy::{prelude::*, ui::*};

#[allow(clippy::type_complexity)]
pub fn pick_up_items(
    mut cmd: Commands,
    mut pick_up_item_reader: EventReader<PickUpItemEvent>,
    mut actors: Query<(&Vector2D, &mut Inventory, &mut Equipment)>,
    items: Query<
        (Entity, &Vector2D, &ItemType, &Children),
        (
            With<Transform>,
            With<GlobalTransform>,
            With<VisibilityToggle>,
        ),
    >,
) {
    for e in pick_up_item_reader.iter() {
        if let Ok((actor_pt, mut inventory, mut equipment)) = actors.get_mut(e.picker) {
            for (item_entity, _, item_type, children) in
                items.iter().filter(|(_, pt, _, _)| **pt == *actor_pt)
            {
                let equiped = equipment.add(item_entity, item_type);
                if equiped || inventory.add(item_entity) {
                    for c in children.iter() {
                        cmd.entity(*c).despawn_recursive();
                    }
                    cmd.entity(item_entity)
                        .remove::<Vector2D>()
                        .remove::<Transform>()
                        .remove::<GlobalTransform>()
                        .remove::<VisibilityToggle>();
                }
                if equiped {
                    cmd.entity(item_entity)
                        .insert(EquipedOwned { id: e.picker });
                    cmd.entity(e.picker).insert(StatsComputedDirty {});
                }
            }
        }
    }
}

pub fn drop_item(
    mut cmd: Commands,
    mut drop_reader: EventReader<DropItemEvent>,
    mut actors: Query<(&Vector2D, &mut Inventory, &mut Equipment)>,
) {
    for e in drop_reader.iter() {
        if let Ok((pt, mut inventory, mut equipment)) = actors.get_mut(e.droper) {
            if inventory.take(e.item) {
                cmd.entity(e.item).insert(*pt);
            } else if equipment.take(e.item) {
                cmd.entity(e.droper).insert(StatsComputedDirty {});
                cmd.entity(e.item).insert(*pt).remove::<EquipedOwned>();
            }
        }
    }
}

pub fn get_gear_image_bundle(
    tile_size: f32,
    top_px: f32,
    left_px: f32,
    image: UiImage,
) -> ImageBundle {
    ImageBundle {
        style: Style {
            size: Size::new(Val::Px(tile_size), Val::Px(tile_size)),
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(top_px),
                left: Val::Px(left_px),
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
    players: Query<(&EquipmentDisplay, &Inventory), With<MovingPlayer>>,
    inventory_display: Query<Entity, With<InventoryDisplay>>,
) {
    if !keys.just_pressed(KeyCode::I) {
        return;
    }
    let (equipment_display, inventory) = if let Ok(player) = players.get_single() {
        player
    } else {
        return;
    };
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
                color: Color::rgba(0., 0., 0., 0.).into(),
                ..default()
            })
            .with_children(|parent| {
                // NOTE: drag area and player gear
                parent
                    .spawn()
                    // equipment display
                    .insert(Name::new("equipment"))
                    .insert_bundle(NodeBundle {
                        focus_policy: FocusPolicy::Pass,
                        style: Style {
                            size: Size::new(Val::Px(256.0), Val::Px(128.)),
                            ..default()
                        },
                        // TODO: should be an image instead of a color
                        color: Color::rgba(0.015, 0.04, 0.025, 0.96).into(),
                        ..default()
                    })
                    .with_children(|cb| {
                        for ((item_type, idx), position) in equipment_display.iter() {
                            cb.spawn()
                                .insert(ItemEquipSlot::new((*item_type, *idx)))
                                .insert_bundle(ImageBundle {
                                    style: Style {
                                        size: Size::new(
                                            Val::Px(map_options.tile_size),
                                            Val::Px(map_options.tile_size),
                                        ),
                                        position_type: PositionType::Absolute,
                                        position: Rect {
                                            left: Val::Px(position.x),
                                            top: Val::Px(position.y),
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    image: inventory_assets.slot.clone().into(),
                                    ..Default::default()
                                });
                        }
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
                        for i in 0..inventory.len() {
                            cb.spawn()
                                .insert(Name::new(format!("Slot {}", i)))
                                .insert(ItemDisplaySlot::new(i))
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

pub fn equipment_update(
    mut cmd: Commands,
    map_options: Res<MapOptions>,
    inventory_assets: Res<InventoryAssets>,
    player_equipment: Query<&Equipment, With<MovingPlayer>>,
    items: Query<&RenderInfo, With<ItemType>>,
    mut item_slots: Query<(Entity, &mut ItemEquipSlot)>,
) {
    let equipment = if let Ok(i) = player_equipment.get_single() {
        i
    } else {
        return;
    };
    for (ee, mut slot) in item_slots.iter_mut() {
        let mut slot_cmd = cmd.entity(ee);
        let mut render_item = false;
        let mut render_dummy = false;
        let mut ui_image = None;
        if let Some(equiped_item) = equipment[slot.index()] {
            if slot.is_dummy_rendered {
                slot_cmd.despawn_descendants();
                slot.is_dummy_rendered = false;
            }
            render_item = if let Some(slot_item) = slot.item {
                if equiped_item != slot_item {
                    slot_cmd.despawn_descendants();
                    slot.item = Some(equiped_item);
                    true
                } else {
                    false
                }
            } else {
                slot.item = Some(equiped_item);
                true
            };
            if let Ok(info) = items.get(equiped_item) {
                ui_image = Some(info.texture.clone().into());
            }
        } else {
            if let Some(_slot_item) = slot.item {
                slot.item = None;
                slot_cmd.despawn_descendants();
                render_dummy = true;
                slot.is_dummy_rendered = true;
            } else {
            }
            if !slot.is_dummy_rendered {
                slot_cmd.despawn_descendants();
                render_dummy = true;
                slot.is_dummy_rendered = true;
            }

            let (item_type, _) = slot.index();
            ui_image = Some(
                match item_type {
                    ItemType::MainHand => inventory_assets.main_hand_gear.clone(),
                    ItemType::OffHand => inventory_assets.off_hand_gear.clone(),
                    ItemType::Head => inventory_assets.head_wear.clone(),
                    ItemType::Neck => inventory_assets.neck_wear.clone(),
                    ItemType::Body => inventory_assets.body_wear.clone(),
                    ItemType::Feet => inventory_assets.feet_wear.clone(),
                    ItemType::Finger => inventory_assets.finger_wear.clone(),
                }
                .into(),
            );
        }
        if render_dummy {
            if let Some(image) = ui_image {
                slot_cmd.with_children(|cb| {
                    cb.spawn().insert_bundle(ImageBundle {
                        style: Style {
                            size: Size::new(
                                Val::Px(map_options.tile_size),
                                Val::Px(map_options.tile_size),
                            ),
                            ..default()
                        },
                        image,
                        color: Color::rgba(1., 1., 1., 0.5).into(),
                        ..Default::default()
                    });
                });
            } else {
                bevy::log::error!("dummy item has no ui image.");
            }
        } else if render_item {
            if let Some(image) = ui_image {
                slot_cmd.with_children(|cb| {
                    cb.spawn().insert_bundle(ImageBundle {
                        style: Style {
                            size: Size::new(
                                Val::Px(map_options.tile_size),
                                Val::Px(map_options.tile_size),
                            ),
                            ..default()
                        },
                        image,
                        ..Default::default()
                    });
                });
            } else {
                bevy::log::error!("item in Equipment but not in the world.");
            }
        }
    }
}

pub fn inventory_update(
    mut cmd: Commands,
    map_options: Res<MapOptions>,
    player_inventory: Query<&Inventory, With<MovingPlayer>>,
    items: Query<&RenderInfo, With<ItemType>>,
    mut item_slots: Query<(Entity, &mut ItemDisplaySlot)>,
) {
    let inventory = if let Ok(i) = player_inventory.get_single() {
        i
    } else {
        return;
    };
    for (ee, mut slot) in item_slots.iter_mut() {
        let mut slot_cmd = cmd.entity(ee);
        if let Some(item) = inventory[slot.index()] {
            let render = if let Some(slot_item) = slot.item {
                if item != slot_item {
                    slot_cmd.despawn_descendants();
                    true
                } else {
                    false
                }
            } else {
                slot.item = Some(item);
                true
            };
            if render {
                if let Ok(info) = items.get(item) {
                    slot_cmd.with_children(|cb| {
                        cb.spawn()
                            .insert(DragableUI::default())
                            .insert(Interaction::default())
                            .insert_bundle(ImageBundle {
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
        } else {
            slot_cmd.despawn_descendants();
        }
    }
}
