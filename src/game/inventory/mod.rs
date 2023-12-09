use bevy::prelude::*;

use self::{
    components::{Inventory, InventorySlot, HotbarSlot},
    systems::{initialize_inventories, test_inputs},
};

pub mod components;
pub mod helpers;
pub mod systems;
pub mod events;
pub mod ui;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Inventory>()
            .register_type::<InventorySlot>()
            .register_type::<HotbarSlot>()
            .add_systems(Update, (initialize_inventories, test_inputs));
    }
}
