use std::marker::PhantomData;

use crate::{
    draggable_ui::{ui_apply_drag_pos, ui_drag_interaction},
    systems::{
        append_world_hovertip, equipment_update, inventory_update, toggle_inventory_open,
        ui_apply_fixed_z, ui_click_item_equip, ui_click_item_unequip, ui_hovertip_interaction,
        world_hovertip_interaction,
    },
    InventoryDisplayToggleEvent, ItemTypeUiImage,
};
use bevy::{ecs::schedule::StateData, prelude::*};
use bevy_inventory::{ItemDropEvent, ItemPickUpEvent, ItemType};

pub struct InventoryUiPlugin<S, I: ItemType, T: ItemTypeUiImage<I>> {
    pub state_running: S,
    pub phantom_1: PhantomData<I>,
    pub phantom_2: PhantomData<T>,
}

impl<S: StateData, I: ItemType, T: ItemTypeUiImage<I>> Plugin for InventoryUiPlugin<S, I, T> {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::First, ui_drag_interaction)
            .add_system_to_stage(
                CoreStage::PostUpdate,
                ui_apply_fixed_z
                    // .after(bevy::ui::update::ui_z_system)
                    .before(bevy::transform::TransformSystem::TransformPropagate),
            )
            .add_system_set(
                SystemSet::on_update(self.state_running.clone())
                    .label("inventory_ui")
                    .with_system(ui_apply_drag_pos)
                    .with_system(append_world_hovertip)
                    .with_system(toggle_inventory_open::<I>)
                    .with_system(
                        equipment_update::<I, T>
                            .after(ui_click_item_equip::<I>)
                            .after(ui_click_item_unequip::<I>),
                    )
                    .with_system(
                        inventory_update::<I>
                            .after(ui_click_item_equip::<I>)
                            .after(ui_click_item_unequip::<I>),
                    )
                    .with_system(ui_hovertip_interaction::<I>)
                    .with_system(world_hovertip_interaction)
                    .with_system(ui_click_item_equip::<I>)
                    .with_system(ui_click_item_unequip::<I>),
            )
            .add_event::<InventoryDisplayToggleEvent>()
            .add_event::<ItemPickUpEvent>()
            .add_event::<ItemDropEvent>();

        bevy::log::info!("Loaded InventoryUiPlugin Plugin");
    }
}
