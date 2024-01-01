use bevy::prelude::*;

use crate::game::world::states::WorldState;

use self::{
    events::CreateInventoryUI,
    systems::{create_inventory_uis, open_inventory},
};

pub mod events;
pub mod systems;

pub struct InventoryUIPlugin;

impl Plugin for InventoryUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CreateInventoryUI>().add_systems(
            Update,
            (create_inventory_uis, open_inventory).run_if(in_state(WorldState::Created)),
        );
    }
}
