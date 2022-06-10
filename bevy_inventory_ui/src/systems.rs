use crate::{
    draggable_ui::DragableUI, inventory_assets::InventoryAssets, EquipmentDisplay,
    EquipmentDisplayNode, EquipmentDisplaySlot, InventoryDisplayNode, InventoryDisplayOptions,
    InventoryDisplayOwner, InventoryDisplaySlot, InventoryDisplayToggleEvent, UiRenderInfo,
};
use bevy::{prelude::*, ui::*};
use bevy_inventory::{Equipment, Inventory, ItemType};

pub fn toggle_inventory_open(
    mut cmd: Commands,
    mut inventory_toggle_reader: EventReader<InventoryDisplayToggleEvent>,
    inventory_assets: Res<InventoryAssets>,
    inventory_options: Res<InventoryDisplayOptions>,
    actors: Query<(&EquipmentDisplay, &Inventory)>,
    inventory_displays: Query<(Entity, &InventoryDisplayOwner)>,
) {
    for e in inventory_toggle_reader.iter() {
        let (equipment_display, inventory) = if let Ok(player) = actors.get(e.id) {
            player
        } else {
            bevy::log::error!("InventoryDisplayToggleEvent with invalif actor_id (missing EquipmentDisplay, Inventory)");
            return;
        };

        if let Some((inventory_display_entity, _)) =
            inventory_displays.iter().find(|(_, o)| o.id == e.id)
        {
            cmd.entity(inventory_display_entity).despawn_recursive();
            return;
        }

        cmd.spawn()
            .insert(Name::new("inventory display"))
            .insert(InventoryDisplayOwner { id: e.id })
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
                // NOTE: equipment with slots
                parent
                    .spawn()
                    // equipment display
                    .insert(Name::new("equipment"))
                    .insert(EquipmentDisplayNode { id: e.id })
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
                        for (&index, position) in equipment_display.iter() {
                            cb.spawn()
                                .insert(Name::new(format!("Equipment display slot {:?}", index)))
                                .insert({
                                    EquipmentDisplaySlot {
                                        index,
                                        item: None,
                                        is_dummy_rendered: false,
                                    }
                                })
                                .insert_bundle(ImageBundle {
                                    style: Style {
                                        size: Size::new(
                                            Val::Px(inventory_options.tile_size),
                                            Val::Px(inventory_options.tile_size),
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
                // NOTE: inventory with slots
                parent
                    .spawn()
                    .insert(Name::new("inventory"))
                    .insert(InventoryDisplayNode { id: e.id })
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
                        for index in 0..inventory.len() {
                            cb.spawn()
                                .insert(Name::new(format!("Inventory display slot {}", index)))
                                .insert(InventoryDisplaySlot { index, item: None })
                                .insert_bundle(ImageBundle {
                                    style: Style {
                                        size: Size::new(
                                            Val::Px(inventory_options.tile_size),
                                            Val::Px(inventory_options.tile_size),
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
    inventory_options: Res<InventoryDisplayOptions>,
    inventory_display_nodes: Query<(&InventoryDisplayNode, &Children)>,
    inventories: Query<&Inventory>,
    items: Query<&UiRenderInfo, With<ItemType>>,
    mut inventory_slots: Query<&mut InventoryDisplaySlot>,
) {
    for (display_node, display_node_children) in inventory_display_nodes.iter() {
        let inventory = if let Ok(inventory) = inventories.get(display_node.id) {
            inventory
        } else {
            bevy::log::error!("InventoryDisplayNode without associated Inventory");
            continue;
        };
        for &slot_entity in display_node_children.iter() {
            let mut slot = if let Ok(slot) = inventory_slots.get_mut(slot_entity) {
                slot
            } else {
                bevy::log::error!(
                    "InventoryDisplayNode's child is not a InventoryDisplaySlot. Should be."
                );
                continue;
            };
            let mut slot_cmd = cmd.entity(slot_entity);
            if let Some(item_entity) = inventory[slot.index] {
                let render = if let Some(slot_item) = slot.item {
                    if item_entity != slot_item {
                        slot_cmd.despawn_descendants();
                        true
                    } else {
                        false
                    }
                } else {
                    slot.item = Some(item_entity);
                    true
                };
                if render {
                    if let Ok(info) = items.get(item_entity) {
                        slot_cmd.with_children(|cb| {
                            // TODO: insert InventoryItemDisplay
                            cb.spawn()
                                .insert(Interaction::default())
                                .insert_bundle(ImageBundle {
                                    style: Style {
                                        size: Size::new(
                                            Val::Px(inventory_options.tile_size),
                                            Val::Px(inventory_options.tile_size),
                                        ),
                                        ..default()
                                    },
                                    image: info.image.clone(),
                                    ..Default::default()
                                });
                        });
                    } else {
                        bevy::log::error!(
                            "item in inventory but not in the world. Or missing UiRenderInfo."
                        );
                    }
                }
            } else {
                slot_cmd.despawn_descendants();
            }
        }
    }
}

pub fn equipment_update(
    mut cmd: Commands,
    map_options: Res<InventoryDisplayOptions>,
    inventory_assets: Res<InventoryAssets>,
    equipment_display_nodes: Query<(&EquipmentDisplayNode, &Children)>,
    equipments: Query<&Equipment>,
    items: Query<&UiRenderInfo, With<ItemType>>,
    mut equipment_slots: Query<&mut EquipmentDisplaySlot>,
) {
    for (display_node, display_node_children) in equipment_display_nodes.iter() {
        let equipment = if let Ok(equipment) = equipments.get(display_node.id) {
            equipment
        } else {
            bevy::log::error!("EquipmentDisplayNode without associated Equipment");
            continue;
        };
        for &slot_entity in display_node_children.iter() {
            let mut slot = if let Ok(slot) = equipment_slots.get_mut(slot_entity) {
                slot
            } else {
                bevy::log::error!(
                    "EquipmentDisplayNode's child is not a EquipmentDisplaySlot. Should be."
                );
                continue;
            };
            let mut slot_cmd = cmd.entity(slot_entity);
            let mut render_item = false;
            let mut render_dummy_item = false;
            let mut ui_image = None;
            if let Some(equiped_item_entity) = equipment[slot.index] {
                if slot.is_dummy_rendered {
                    slot_cmd.despawn_descendants();
                    slot.is_dummy_rendered = false;
                }
                render_item = if let Some(slot_item_entity) = slot.item {
                    if equiped_item_entity != slot_item_entity {
                        slot_cmd.despawn_descendants();
                        slot.item = Some(equiped_item_entity);
                        true
                    } else {
                        false
                    }
                } else {
                    slot.item = Some(equiped_item_entity);
                    true
                };
                if let Ok(info) = items.get(equiped_item_entity) {
                    ui_image = Some(info.image.clone());
                }
            } else {
                if let Some(_slot_item_entity) = slot.item {
                    slot.item = None;
                    slot_cmd.despawn_descendants();
                    render_dummy_item = true;
                    slot.is_dummy_rendered = true;
                }
                if !slot.is_dummy_rendered {
                    slot_cmd.despawn_descendants();
                    render_dummy_item = true;
                    slot.is_dummy_rendered = true;
                }

                let (item_type, _) = slot.index;
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
            if render_dummy_item {
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
                        // TODO: insert EquipmentItemDisplay
                        cb.spawn()
                            .insert(Interaction::default())
                            .insert_bundle(ImageBundle {
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
                    bevy::log::error!(
                        "item in Equipment but not in the world. Or missing UiRenderInfo."
                    );
                }
            }
        }
    }
}
