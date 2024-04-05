use std::marker::PhantomData;

use crate::{
    draggable_ui::{ui_apply_drag_pos, ui_drag_interaction},
    systems::{
        append_world_hovertip, equipment_update, inventory_update, toggle_inventory_open,
        ui_click_item_equip, ui_click_item_unequip, ui_hovertip_interaction,
        world_hovertip_interaction,
    },
    InventoryDisplayToggleEvent, ItemTypeUiImage,
};
use bevy::prelude::*;
use bevy_inventory::{ItemDropEvent, ItemPickUpEvent, ItemType};
use bevy_roguelike_states::AppState;

pub struct InventoryUiPlugin<I: ItemType, T: ItemTypeUiImage<I>> {
    _phantom: PhantomData<(I, T)>,
}

impl<I: ItemType, T: ItemTypeUiImage<I>> Default for InventoryUiPlugin<I, T> {
    fn default() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<I: ItemType, T: ItemTypeUiImage<I>> Plugin for InventoryUiPlugin<I, T> {
    fn build(&self, app: &mut App) {
        app.add_system(ui_drag_interaction.in_base_set(CoreSet::First))
            .add_systems(
                (
                    ui_apply_drag_pos.run_if(in_state(AppState::InGame)),
                    append_world_hovertip.run_if(in_state(AppState::InGame)),
                    toggle_inventory_open::<I>.run_if(in_state(AppState::InGame)),
                    equipment_update::<I, T>
                        .after(ui_click_item_equip::<I>)
                        .after(ui_click_item_unequip::<I>)
                        .run_if(in_state(AppState::InGame)),
                    inventory_update::<I>
                        .after(ui_click_item_equip::<I>)
                        .after(ui_click_item_unequip::<I>)
                        .run_if(in_state(AppState::InGame)),
                    ui_hovertip_interaction::<I>.run_if(in_state(AppState::InGame)),
                    world_hovertip_interaction.run_if(in_state(AppState::InGame)),
                    ui_click_item_equip::<I>.run_if(in_state(AppState::InGame)),
                    ui_click_item_unequip::<I>.run_if(in_state(AppState::InGame)),
                )
                    .in_base_set(CoreSet::Update),
            )
            .add_event::<InventoryDisplayToggleEvent>();

        bevy::log::info!("Loaded InventoryUiPlugin Plugin");
    }
}
