use bevy::prelude::Event;

use super::components::SlotEntity;

#[derive(Event, Debug)]
pub struct UpdateInventorySlot {
    pub inventory_slot: SlotEntity,
}
