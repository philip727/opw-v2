use bevy::prelude::*;

use crate::{game::world::states::WorldState, states::AppState};

use self::{
    events::CreateInventoryUI,
    resources::InventoryUIManager,
    systems::{
        cleanup_inventories_root_ui, create_inventories_root_ui, open_inventory,
        toggle_inventory_ui, update_inventory_ui,
    },
};

pub mod events;
pub mod helpers;
pub mod resources;
pub mod systems;
pub mod components;

pub struct InventoryUIPlugin;

impl Plugin for InventoryUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CreateInventoryUI>()
            .init_resource::<InventoryUIManager>()
            .add_systems(OnEnter(AppState::InGame), create_inventories_root_ui)
            .add_systems(OnExit(AppState::InGame), cleanup_inventories_root_ui)
            .add_systems(
                Update,
                (toggle_inventory_ui, open_inventory, update_inventory_ui).run_if(in_state(WorldState::Created)),
            );
    }
}
