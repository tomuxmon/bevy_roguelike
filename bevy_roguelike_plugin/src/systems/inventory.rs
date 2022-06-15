use crate::components::*;
use bevy::{prelude::*, utils::HashMap};
use bevy_inventory::{Equipment, Inventory, ItemDropEvent, ItemPickUpEvent, ItemType};
use bevy_inventory_ui::{EquipmentDisplay, InventoryDisplayToggleEvent, ItemUiTextInfo};

#[allow(clippy::type_complexity)]
pub fn item_fill_text_info<I: ItemType>(
    mut cmd: Commands,
    items: Query<
        (
            Entity,
            &Name,
            Option<&Attributes>,
            Option<&Protection>,
            Option<&Resistance>,
            Option<&Damage>,
            Option<&Block>,
        ),
        (With<I>, Without<ItemUiTextInfo>),
    >,
) {
    for (entity, name, attributes, protection, resistance, damage, block) in items.iter() {
        let mut infos = HashMap::default();
        if let Some(attributes) = attributes {
            infos
                .entry("Attributes".to_string())
                .insert(format!("{}", attributes));
        }
        if let Some(protection) = protection {
            infos
                .entry("Protection".to_string())
                .insert(format!("{}", protection));
        }
        if let Some(resistance) = resistance {
            infos
                .entry("Resistance".to_string())
                .insert(format!("{}", resistance));
        }
        if let Some(damage) = damage {
            infos
                .entry("Damage".to_string())
                .insert(format!("{:?} ({})", damage.amount, damage.kind));
            infos
                .entry("Hit rate".to_string())
                .insert(damage.hit_chance.amount.to_string());
            infos
                .entry("Hit cost".to_string())
                .insert(damage.hit_cost.cost.to_string());
        }
        if let Some(block) = block {
            infos.entry("Block type".to_string()).insert(
                block
                    .block_type
                    .iter()
                    .map(|p| format!("{}", p))
                    .fold("".to_string(), |acc, x| format!("{},{}", x, acc)),
            );
            infos
                .entry("Block rate".to_string())
                .insert(block.chance.amount.to_string());
        }
        cmd.entity(entity).insert(ItemUiTextInfo {
            name: name.as_str().to_string(),
            infos,
        });
    }
}

#[allow(clippy::type_complexity)]
pub fn pick_up_items<I: ItemType>(
    mut cmd: Commands,
    mut pick_up_item_reader: EventReader<ItemPickUpEvent>,
    mut actors: Query<(&Vector2D, &mut Inventory, &mut Equipment<I>)>,
    items: Query<
        (Entity, &Vector2D, &I, &Children),
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
                if equipment.add(item_entity, item_type) || inventory.add(item_entity) {
                    for c in children.iter() {
                        cmd.entity(*c).despawn_recursive();
                    }
                    cmd.entity(item_entity)
                        .remove::<Vector2D>()
                        .remove::<Transform>()
                        .remove::<GlobalTransform>()
                        .remove::<VisibilityToggle>();
                }
            }
        }
    }
}

pub fn drop_item<I: ItemType>(
    mut cmd: Commands,
    mut drop_reader: EventReader<ItemDropEvent>,
    mut actors: Query<(&Vector2D, &mut Inventory, &mut Equipment<I>)>,
) {
    for e in drop_reader.iter() {
        if let Ok((pt, mut inventory, mut equipment)) = actors.get_mut(e.droper) {
            if inventory.take(e.item) || equipment.take(e.item) {
                cmd.entity(e.item).insert(*pt);
            }
        }
    }
}

pub fn equip_owned_add<I: ItemType>(
    mut cmd: Commands,
    equipments: Query<(Entity, &Equipment<I>)>,
    items: Query<Entity, (With<I>, Without<ItemEquipedOwned>)>,
) {
    for (actor_entity, equipment) in equipments.iter() {
        for (_, item_entity) in equipment.iter_some() {
            if items.get(item_entity).is_ok() {
                cmd.entity(item_entity).insert(ItemEquipedOwned {
                    actor: actor_entity,
                });
                cmd.entity(actor_entity).insert(StatsComputedDirty {});
            }
        }
    }
}

pub fn equip_owned_remove<I: ItemType>(
    mut cmd: Commands,
    equipments: Query<(Entity, &Equipment<I>)>,
    items: Query<(Entity, &ItemEquipedOwned), With<I>>,
) {
    for (item_entity, owned) in items.iter() {
        if let Ok((actor_entity, equipment)) = equipments.get(owned.actor) {
            if !equipment.iter_some().any(|(_, e)| e == item_entity) {
                cmd.entity(item_entity).remove::<ItemEquipedOwned>();
                cmd.entity(actor_entity).insert(StatsComputedDirty {});
            }
        } else {
            cmd.entity(item_entity).remove::<ItemEquipedOwned>();
            bevy::log::error!("actor entity not found. Could not add StatsComputedDirty");
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn toggle_inventory_open_event_send<I: ItemType>(
    keys: Res<Input<KeyCode>>,
    players: Query<
        Entity,
        (
            With<MovingPlayer>,
            With<EquipmentDisplay<I>>,
            With<Inventory>,
        ),
    >,
    mut inventory_toggle_writer: EventWriter<InventoryDisplayToggleEvent>,
) {
    if !keys.just_pressed(KeyCode::I) {
        return;
    }
    if let Ok(player) = players.get_single() {
        inventory_toggle_writer.send(InventoryDisplayToggleEvent { actor: player });
    }
}
