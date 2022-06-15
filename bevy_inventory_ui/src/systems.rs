use crate::{
    assets::InventoryUiAssets, draggable_ui::DragableUI, Equipable, EquipmentDisplay,
    EquipmentDisplayNode, EquipmentDisplaySlot, HoverTip, InventoryDisplayNode,
    InventoryDisplayOptions, InventoryDisplayOwner, InventoryDisplaySlot,
    InventoryDisplayToggleEvent, ItemTypeUiImage, ItemUiTextInfo, UiRenderInfo, Unequipable,
};
use bevy::{prelude::*, ui::*};
use bevy_inventory::{Equipment, Inventory, ItemDropEvent, ItemType};

pub(crate) fn toggle_inventory_open<I: ItemType>(
    mut cmd: Commands,
    mut inventory_toggle_reader: EventReader<InventoryDisplayToggleEvent>,
    slot_asset: Res<InventoryUiAssets>,
    inventory_options: Res<InventoryDisplayOptions>,
    actors: Query<(&EquipmentDisplay<I>, &Inventory)>,
    inventory_displays: Query<(Entity, &InventoryDisplayOwner)>,
) {
    for e in inventory_toggle_reader.iter() {
        let (equipment_display, inventory) = if let Ok(player) = actors.get(e.actor) {
            player
        } else {
            bevy::log::error!("InventoryDisplayToggleEvent with invalif actor_id (missing EquipmentDisplay, Inventory)");
            return;
        };

        if let Some((inventory_display_entity, _)) =
            inventory_displays.iter().find(|(_, o)| o.id == e.actor)
        {
            cmd.entity(inventory_display_entity).despawn_recursive();
            return;
        }

        cmd.spawn()
            .insert(Name::new("inventory display"))
            .insert(InventoryDisplayOwner { id: e.actor })
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
                    .insert(EquipmentDisplayNode { actor: e.actor })
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
                                    image: slot_asset.slot.clone().into(),
                                    ..Default::default()
                                });
                        }
                    });
                // NOTE: inventory with slots
                parent
                    .spawn()
                    .insert(Name::new("inventory"))
                    .insert(InventoryDisplayNode { id: e.actor })
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
                                    image: slot_asset.slot.clone().into(),
                                    ..Default::default()
                                });
                        }
                    });
            });
    }
}

pub(crate) fn inventory_update<I: ItemType>(
    mut cmd: Commands,
    inventory_options: Res<InventoryDisplayOptions>,
    inventory_display_nodes: Query<(&InventoryDisplayNode, &Children)>,
    inventories: Query<&Inventory>,
    items: Query<&UiRenderInfo, With<I>>,
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
                        slot.item = Some(item_entity);
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
                            cb.spawn()
                                .insert(Interaction::default())
                                .insert(Equipable {
                                    actor: display_node.id,
                                    item: item_entity,
                                })
                                .insert(HoverTip::new(item_entity))
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
                slot.item = None;
                slot_cmd.despawn_descendants();
            }
        }
    }
}

pub(crate) fn equipment_update<I: ItemType, T: ItemTypeUiImage<I>>(
    mut cmd: Commands,
    inventory_display_options: Res<InventoryDisplayOptions>,
    inventory_assets: Res<T>,
    equipment_display_nodes: Query<(&EquipmentDisplayNode, &Children)>,
    equipments: Query<&Equipment<I>>,
    items: Query<&UiRenderInfo, With<I>>,
    mut equipment_slots: Query<&mut EquipmentDisplaySlot<I>>,
) {
    for (display_node, display_node_children) in equipment_display_nodes.iter() {
        let equipment = if let Ok(equipment) = equipments.get(display_node.actor) {
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
                ui_image = Some(inventory_assets.get_image(item_type));
            }
            if render_dummy_item {
                if let Some(image) = ui_image {
                    slot_cmd.with_children(|cb| {
                        cb.spawn().insert_bundle(ImageBundle {
                            style: Style {
                                size: Size::new(
                                    Val::Px(inventory_display_options.tile_size),
                                    Val::Px(inventory_display_options.tile_size),
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
                        cb.spawn()
                            .insert(Interaction::default())
                            .insert(Unequipable {
                                actor: display_node.actor,
                                item: slot.item.unwrap(),
                            })
                            .insert(HoverTip::new(slot.item.unwrap()))
                            .insert_bundle(ImageBundle {
                                style: Style {
                                    size: Size::new(
                                        Val::Px(inventory_display_options.tile_size),
                                        Val::Px(inventory_display_options.tile_size),
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

pub(crate) fn ui_hovertip_interaction<I: ItemType>(
    mut cmd: Commands,
    inventory_display_options: Res<InventoryDisplayOptions>,
    inventory_ui_assets: Res<InventoryUiAssets>,
    mut interactive_hovertip: Query<(Entity, &GlobalTransform, &Interaction, &mut HoverTip)>,
    items: Query<&ItemUiTextInfo, With<I>>,
) {
    // TODO: use transform to figure out where to draw a tooltip
    // above slot
    // below slot
    for (entity, _transform, &interaction, mut hovet_tip) in interactive_hovertip.iter_mut() {
        if interaction == Interaction::Hovered && !hovet_tip.hovered {
            cmd.entity(entity).with_children(|cb| {
                let image = inventory_ui_assets.hover_cursor_image.clone().into();
                cb.spawn().insert_bundle(ImageBundle {
                    style: Style {
                        size: Size::new(
                            Val::Px(inventory_display_options.tile_size),
                            Val::Px(inventory_display_options.tile_size),
                        ),

                        ..default()
                    },
                    image,
                    focus_policy: FocusPolicy::Pass,
                    ..Default::default()
                });
            });
            hovet_tip.hovered = true;
        } else if interaction != Interaction::Hovered && hovet_tip.hovered {
            cmd.entity(entity).despawn_descendants();
            hovet_tip.hovered = false;
        }
        if hovet_tip.hovered && !hovet_tip.tooltip_shown {
            if let Ok(info) = items.get(hovet_tip.item_entity) {
                cmd.entity(entity).with_children(|cb| {
                    cb.spawn()
                        .insert_bundle(NodeBundle {
                            style: Style {
                                flex_wrap: FlexWrap::WrapReverse,
                                flex_direction: FlexDirection::Row,
                                size: Size::new(Val::Auto, Val::Auto),
                                // max_size does not work :|
                                max_size: Size::new(Val::Px(128.), Val::Px(256.)),
                                min_size: Size::new(Val::Px(32.), Val::Px(32.)),
                                position_type: PositionType::Absolute,
                                position: Rect {
                                    left: Val::Px(inventory_display_options.tile_size),
                                    bottom: Val::Px(inventory_display_options.tile_size),
                                    ..default()
                                },
                                ..default()
                            },
                            color: Color::rgba(0.015, 0.04, 0.025, 0.96).into(),
                            ..default()
                        })
                        .with_children(|cb| {
                            cb.spawn().insert_bundle(TextBundle {
                                style: Style {
                                    margin: Rect::all(Val::Px(4.0)),
                                    ..default()
                                },
                                text: Text::with_section(
                                    info.name.as_str(),
                                    TextStyle {
                                        font: inventory_ui_assets.font.clone(),
                                        font_size: 24.0,
                                        color: Color::WHITE,
                                    },
                                    Default::default(),
                                ),
                                ..default()
                            });
                            for (title, description) in info.infos.iter() {
                                cb.spawn().insert_bundle(TextBundle {
                                    style: Style {
                                        margin: Rect::all(Val::Px(2.0)),
                                        ..default()
                                    },
                                    text: Text::with_section(
                                        format!("{}: {}", title, description),
                                        TextStyle {
                                            font: inventory_ui_assets.font.clone(),
                                            font_size: 18.0,
                                            color: Color::WHITE,
                                        },
                                        Default::default(),
                                    ),
                                    ..default()
                                });
                            }
                        });
                });
            } else {
                bevy::log::error!(
                    "Item entity must have ItemUiTextInfo present in order to show a tooltip."
                );
            }
            hovet_tip.tooltip_shown = true;
        } else if !hovet_tip.hovered && hovet_tip.tooltip_shown {
            hovet_tip.tooltip_shown = false;
        }
    }
}

pub(crate) fn ui_click_item_equip<I: ItemType>(
    interactive_equipables: Query<(&Interaction, &Equipable)>,
    mut actors: Query<(&mut Inventory, &mut Equipment<I>)>,
    items: Query<&I>,
) {
    for (interaction, equipable) in interactive_equipables.iter() {
        if *interaction == Interaction::Clicked {
            if let Ok((mut inventory, mut equipment)) = actors.get_mut(equipable.actor) {
                let item_type = if let Ok(item_type) = items.get(equipable.item) {
                    item_type
                } else {
                    bevy::log::error!("item with no type");
                    continue;
                };
                if inventory.take(equipable.item) {
                    if !equipment.add(equipable.item, item_type) {
                        inventory.add(equipable.item);
                        bevy::log::info!("could not equip item placing back into inventory");
                    }
                } else {
                    bevy::log::error!("Equipable Item not in inventory.");
                }
            }
        }
    }
}

pub(crate) fn ui_click_item_unequip<I: ItemType>(
    interactive_unequipables: Query<(&Interaction, &Unequipable)>,
    mut actors: Query<(&mut Inventory, &mut Equipment<I>)>,
    mut drop_writer: EventWriter<ItemDropEvent>,
) {
    for (interaction, unequipable) in interactive_unequipables.iter() {
        if *interaction == Interaction::Clicked {
            if let Ok((mut inventory, mut equipment)) = actors.get_mut(unequipable.actor) {
                if equipment.take(unequipable.item) {
                    if !inventory.add(unequipable.item) {
                        drop_writer.send(ItemDropEvent {
                            droper: unequipable.actor,
                            item: unequipable.item,
                        });
                        bevy::log::info!(
                            "could not place unequiped item into inventory. dropping it."
                        );
                    }
                } else {
                    bevy::log::error!("Unequipable Item not in Equipment.");
                }
            }
        }
    }
}
