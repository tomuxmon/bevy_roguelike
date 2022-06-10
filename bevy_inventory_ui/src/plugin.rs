use crate::{
    draggable_ui::{ui_apply_drag_pos, ui_drag_interaction},
    systems::{equipment_update, inventory_update, toggle_inventory_open},
    InventoryDisplayToggleEvent, InventoryTheme,
};
use bevy::{ecs::schedule::StateData, prelude::*};
use bevy_asset_ron::RonAssetPlugin;

pub struct InventoryUiPlugin<T> {
    pub state_running: T,
}

impl<T: StateData> Plugin for InventoryUiPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_plugin(RonAssetPlugin::<InventoryTheme>::new(&[
            "inventorytheme.ron",
        ]))
        .add_system_to_stage(CoreStage::First, ui_drag_interaction)
        .add_system_set(
            SystemSet::on_update(self.state_running.clone())
                .with_system(ui_apply_drag_pos)
                .with_system(toggle_inventory_open)
                .with_system(equipment_update)
                .with_system(inventory_update),
        )
        .add_event::<InventoryDisplayToggleEvent>();

        bevy::log::info!("Loaded InventoryUiPlugin Plugin");
    }
}
