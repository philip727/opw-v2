use bevy::prelude::*;

use crate::game::inventory::helpers::InventoryEntity;

#[derive(Event)]
pub struct CreateInventoryUI {
    pub inventory_entity: InventoryEntity,
    pub ui_inventory_parent: Entity,
}
