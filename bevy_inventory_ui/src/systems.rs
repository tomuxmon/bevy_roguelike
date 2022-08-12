use crate::{
    assets::InventoryUiAssets, draggable_ui::DragableUI, Equipable, EquipmentDisplay,
    EquipmentDisplayNode, EquipmentDisplaySlot, InventoryDisplayNode, InventoryDisplayOptions,
    InventoryDisplayOwner, InventoryDisplaySlot, InventoryDisplayToggleEvent, ItemTypeUiImage,
    UiFixedZ, UiHoverTip, UiRenderInfo, UiTextInfo, Unequipable, WorldHoverTip,
};
use bevy::{ecs::system::EntityCommands, prelude::*, render::camera::RenderTarget, ui::*};
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
            inventory_displays.iter().find(|(_, o)| o.actor == e.actor)
        {
            cmd.entity(inventory_display_entity).despawn_recursive();
            return;
        }
        // TODO: local resource to track inventory position and reopen on the same position.
        cmd.spawn()
            .insert(Name::new("inventory display"))
            .insert(InventoryDisplayOwner { actor: e.actor })
            .insert(DragableUI::default())
            .insert(Interaction::default())
            .insert_bundle(NodeBundle {
                style: Style {
                    flex_wrap: FlexWrap::Wrap,
                    flex_direction: FlexDirection::ColumnReverse,
                    size: Size::new(Val::Px(256.0), Val::Auto),
                    position_type: PositionType::Absolute,
                    position: UiRect {
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
                                        position: UiRect {
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
                                .insert(UiHoverTip::new(item_entity))
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
                            .insert(UiHoverTip::new(slot.item.unwrap()))
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

/// system must be executed in the last core stage in order to override bevy_ui z-order calculation
pub(crate) fn ui_apply_fixed_z(
    mut node_query: Query<(&mut Transform, &mut GlobalTransform, &UiFixedZ), With<Node>>,
) {
    for (mut transform, mut global_transform, fixed) in node_query.iter_mut() {
        transform.translation.z = fixed.z;
        let mut tr = global_transform.compute_transform();
        tr.translation.z = fixed.z;
        *global_transform = GlobalTransform::from(tr);
    }
}

pub(crate) fn ui_hovertip_interaction<I: ItemType>(
    mut cmd: Commands,
    windows: Res<Windows>,
    inventory_display_options: Res<InventoryDisplayOptions>,
    inventory_ui_assets: Res<InventoryUiAssets>,
    mut interactive_hovertip: Query<(Entity, &GlobalTransform, &Interaction, &mut UiHoverTip)>,
    items: Query<&UiTextInfo, With<I>>,
) {
    let window = if let Some(window) = windows.get_primary() {
        window
    } else {
        bevy::log::error!("no window in ui_hovertip_interaction");
        return;
    };

    for (entity, transform, &interaction, mut hover_tip) in interactive_hovertip.iter_mut() {
        let x_first_half = transform.affine().translation.x < window.width() / 2.;
        let y_first_half = transform.affine().translation.y < window.height() / 2.;

        // TODO: extract function to show hovertip

        if interaction == Interaction::Hovered && !hover_tip.hovered {
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
            hover_tip.hovered = true;
        } else if interaction != Interaction::Hovered && hover_tip.hovered {
            cmd.entity(entity).despawn_descendants();
            hover_tip.hovered = false;
        }
        if hover_tip.hovered && !hover_tip.tooltip_shown {
            if let Ok(info) = items.get(hover_tip.tip_owner) {
                cmd.entity(entity).with_children(|cb| {
                    insert_tooltip(
                        cb.spawn(),
                        info,
                        &inventory_ui_assets.font,
                        inventory_display_options.tile_size,
                        x_first_half,
                        y_first_half,
                    );
                });
            } else {
                bevy::log::error!(
                    "Item entity must have ItemUiTextInfo present in order to show a tooltip."
                );
            }
            hover_tip.tooltip_shown = true;
        } else if !hover_tip.hovered && hover_tip.tooltip_shown {
            hover_tip.tooltip_shown = false;
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
                        let mut swap_done = false;
                        let some_item_entity = if let Some((_, entity)) =
                            equipment.iter_some().find(|((it, _), _)| item_type == it)
                        {
                            Some(entity)
                        } else {
                            None
                        };
                        if let Some(entity) = some_item_entity {
                            if equipment.take(entity)
                                && inventory.add(entity)
                                && equipment.add(equipable.item, item_type)
                            {
                                swap_done = true;
                            }
                        }
                        if !swap_done {
                            inventory.add(equipable.item);
                            bevy::log::info!("could not equip item placing back into inventory");
                        }
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

#[allow(clippy::type_complexity)]
pub(crate) fn append_world_hovertip(
    mut cmd: Commands,
    no_ui_things: Query<Entity, (Without<Node>, Without<WorldHoverTip>, With<UiTextInfo>)>,
) {
    for entity in no_ui_things.iter() {
        cmd.entity(entity).insert(WorldHoverTip::default());
    }
}

pub(crate) fn world_hovertip_interaction(
    mut cmd: Commands,
    windows: Res<Windows>,
    inventory_display_options: Res<InventoryDisplayOptions>,
    inventory_ui_assets: Res<InventoryUiAssets>,
    cameras_2d: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut no_ui_things: Query<(&GlobalTransform, &UiTextInfo, &mut WorldHoverTip), Without<Node>>,
) {
    let (cam2d, cam2d_transform) = cameras_2d.single();
    let window = if let RenderTarget::Window(id) = cam2d.target {
        windows.get(id)
    } else {
        windows.get_primary()
    };
    let window = if let Some(window) = window {
        window
    } else {
        bevy::log::error!("could not get any window in world_hovertip_interaction");
        return;
    };
    let cursor_position = window.cursor_position();
    let window_size = Vec2::new(window.width(), window.height());

    // TODO: show a paged tooltip when more then
    // one item at the same hover position
    for (transform, info, mut hover_tip) in no_ui_things.iter_mut() {
        let is_hovered = if let Some(cursor_screen_pos) = cursor_position {
            // NOTE: cursor_world_pos compute stolen from https://bevy-cheatbook.github.io/cookbook/cursor2world.html
            let cursor_world_pos = (cam2d_transform.compute_matrix()
                * cam2d.projection_matrix().inverse())
            .project_point3(((cursor_screen_pos / window_size) * 2.0 - Vec2::ONE).extend(-1.0));

            let thing_pos = transform.affine().translation.truncate();
            let extents = inventory_display_options.tile_size / 2.0;
            let min = thing_pos - extents;
            let max = thing_pos + extents;

            (min.x..max.x).contains(&cursor_world_pos.x)
                && (min.y..max.y).contains(&cursor_world_pos.y)
        } else {
            false
        };

        let ui_screen_pos = transform.affine().translation.truncate()
            - cam2d_transform.affine().translation.truncate()
            + (window_size / 2.)
            - (inventory_display_options.tile_size / 2.);

        let x_first_half = ui_screen_pos.x < window.width() / 2.;
        let y_first_half = ui_screen_pos.y < window.height() / 2.;

        if is_hovered && !hover_tip.hovered {
            let image = inventory_ui_assets.hover_cursor_image.clone().into();
            let id = cmd
                .spawn()
                .insert_bundle(ImageBundle {
                    style: Style {
                        size: Size::new(
                            Val::Px(inventory_display_options.tile_size),
                            Val::Px(inventory_display_options.tile_size),
                        ),
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            left: Val::Px(ui_screen_pos.x),
                            bottom: Val::Px(ui_screen_pos.y),
                            ..default()
                        },
                        ..default()
                    },
                    image,
                    focus_policy: FocusPolicy::Pass,
                    ..Default::default()
                })
                .id();

            hover_tip.tip_entity = Some(id);
            hover_tip.hovered = true;
        } else if !is_hovered && hover_tip.hovered {
            if let Some(entity) = hover_tip.tip_entity {
                cmd.entity(entity).despawn_recursive();
                hover_tip.tip_entity = None;
            }
            hover_tip.hovered = false;
        }
        if hover_tip.hovered && !hover_tip.tooltip_shown {
            if let Some(tip_entity) = hover_tip.tip_entity {
                cmd.entity(tip_entity).with_children(|cb| {
                    insert_tooltip(
                        cb.spawn(),
                        info,
                        &inventory_ui_assets.font,
                        inventory_display_options.tile_size,
                        x_first_half,
                        y_first_half,
                    );
                });
            }
            hover_tip.tooltip_shown = true;
        } else if !hover_tip.hovered && hover_tip.tooltip_shown {
            hover_tip.tooltip_shown = false;
        }
    }
}

fn insert_tooltip(
    mut ec: EntityCommands,
    info: &UiTextInfo,
    font: &Handle<Font>,
    tile_size: f32,
    x_first_half: bool,
    y_first_half: bool,
) {
    ec.insert(UiFixedZ { z: 10. })
        .insert_bundle(NodeBundle {
            style: Style {
                flex_wrap: FlexWrap::WrapReverse,
                flex_direction: FlexDirection::Row,
                min_size: Size::new(Val::Px(128.), Val::Auto),
                max_size: Size::new(Val::Px(256.), Val::Auto),
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: if !y_first_half {
                        Val::Px(tile_size)
                    } else {
                        Val::Undefined
                    },
                    bottom: if y_first_half {
                        Val::Px(tile_size)
                    } else {
                        Val::Undefined
                    },
                    right: if !x_first_half {
                        Val::Px(tile_size)
                    } else {
                        Val::Undefined
                    },
                    left: if x_first_half {
                        Val::Px(tile_size)
                    } else {
                        Val::Undefined
                    },
                },
                ..default()
            },
            color: Color::rgba(0.015, 0.04, 0.025, 0.96).into(),
            ..default()
        })
        .with_children(|cb| {
            cb.spawn()
                .insert(UiFixedZ { z: 11. })
                .insert_bundle(TextBundle {
                    style: Style {
                        margin: UiRect::all(Val::Px(4.0)),
                        ..default()
                    },
                    text: Text::from_section(
                        info.name.as_str(),
                        TextStyle {
                            font: font.clone(),
                            font_size: 24.0,
                            color: Color::WHITE,
                        },
                    ),
                    ..default()
                });
            for (title, description) in info.titles_descriptions.iter() {
                cb.spawn()
                    .insert(UiFixedZ { z: 12. })
                    .insert_bundle(TextBundle {
                        style: Style {
                            margin: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        text: Text::from_section(
                            format!("{}: {}", title, description),
                            TextStyle {
                                font: font.clone(),
                                font_size: 18.0,
                                color: Color::WHITE,
                            },
                        ),
                        ..default()
                    });
            }
        });
}
