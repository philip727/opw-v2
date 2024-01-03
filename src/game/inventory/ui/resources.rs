use bevy::prelude::*;
use bevy::reflect::List;
use bevy::utils::{EntityHashMap, HashMap};

use crate::game::inventory::components::SlotEntity;
use crate::game::inventory::helpers::InventoryEntity;

use super::helpers::{UIInventoryEntity, UISlotEntity};

#[derive(Resource, Default)]
pub struct InventoryUIManager {
    pub ui_entity_to_inventory_entity: EntityHashMap<UIInventoryEntity, InventoryEntity>,
    pub inventory_entity_to_ui_entity: EntityHashMap<InventoryEntity, UIInventoryEntity>,
    pub open_inventories: Vec<InventoryEntity>,
    pub inventory_parent: Option<Entity>,
}

impl InventoryUIManager {
    /// Adds a reference to the inventory entity in the UI manager
    pub fn add_inventory(
        &mut self,
        inventory_entity: InventoryEntity,
        ui_entity: UIInventoryEntity,
    ) {
        self.open_inventories.push(inventory_entity);

        self.inventory_entity_to_ui_entity
            .insert(inventory_entity, ui_entity);
        self.ui_entity_to_inventory_entity
            .insert(ui_entity, inventory_entity);
    }

    /// Removes all references in the UI manager to that inventory entity
    pub fn destroy_inventory(&mut self, inventory_entity: &InventoryEntity) {
        self.open_inventories.retain(|&x| x != *inventory_entity);
        let ui_inventory_entity = self
            .inventory_entity_to_ui_entity
            .get(inventory_entity)
            .unwrap();

        self.ui_entity_to_inventory_entity
            .remove(ui_inventory_entity);

        self.inventory_entity_to_ui_entity.remove(inventory_entity);
    }

    pub fn root(&self) -> Entity {
        self.inventory_parent.unwrap()
    }

    pub fn is_inventory_open(&self, inventory_entity: &InventoryEntity) -> bool {
        self.open_inventories.contains(inventory_entity)
    }

    pub fn ui_to_inventory(&self, ui_entity: &UIInventoryEntity) -> Option<&InventoryEntity> {
        self.ui_entity_to_inventory_entity.get(ui_entity)
    }

    pub fn inventory_to_ui(&self, inv_entity: &InventoryEntity) -> Option<&UIInventoryEntity> {
        self.inventory_entity_to_ui_entity.get(inv_entity)
    }
}
