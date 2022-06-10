use crate::{components::*, events::*};
use bevy::prelude::*;
use bevy_inventory::{Equipment, Inventory, ItemType};
use bevy_inventory_ui::{EquipmentDisplay, InventoryDisplayToggleEvent};

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

#[allow(clippy::type_complexity)]
pub fn toggle_inventory_open_event_send(
    keys: Res<Input<KeyCode>>,
    players: Query<Entity, (With<MovingPlayer>, With<EquipmentDisplay>, With<Inventory>)>,
    mut inventory_toggle_writer: EventWriter<InventoryDisplayToggleEvent>,
) {
    if !keys.just_pressed(KeyCode::I) {
        return;
    }
    if let Ok(player) = players.get_single() {
        inventory_toggle_writer.send(InventoryDisplayToggleEvent { id: player });
    }
}
