use crate::{components::*, events::*};
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
                for c in children.iter() {
                    cmd.entity(*c).despawn_recursive();
                }
                cmd.entity(item_entity)
                    .remove::<Vector2D>()
                    .remove::<Transform>()
                    .remove::<GlobalTransform>()
                    .remove::<VisibilityToggle>();

                inventory.insert(item_entity);
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
            if let Some(_) = inventory.take(&e.item) {
                cmd.entity(e.item).insert(*pt);
            }
        }
    }
}
