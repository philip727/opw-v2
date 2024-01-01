use bevy::prelude::*;

#[derive(Event)]
pub struct CreateInventoryUI {
    pub inventory_entity: Entity,
}
