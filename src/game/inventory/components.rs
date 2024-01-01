use belly::core::relations::props::PropertyProtocol;
use bevy::{prelude::*, utils::HashMap};
use bevy_inspector_egui::prelude::*;

use crate::game::items::{
    components::Item,
    helpers::{BaseItem, ItemRecord},
};

use super::helpers::ItemSlot;

pub type ItemEntity = Entity;
pub type SlotEntity = Entity;

#[derive(Component, InspectorOptions, Reflect, Default)]
#[reflect(Component)]
pub struct Inventory {
    pub slots: Vec<SlotEntity>,
    size: u32,
    initialized: bool,
}

impl Inventory {
    pub fn new(size: u32) -> Self {
        Inventory {
            slots: Vec::new(),
            size,
            initialized: false,
        }
    }

    pub fn initialize(&mut self, inventory_entity: Entity, commands: &mut Commands) {
        for slot_num in 0u32..self.size {
            let entity = commands
                .spawn((
                    Name::new(format!("Inventory Slot {slot_num}")),
                    InventorySlot {
                        item: None,
                        amount: 0,
                    },
                ))
                .id();

            // Adds the inventory slot to the inventory parent entity
            commands.entity(inventory_entity).push_children(&[entity]);
            self.slots.push(entity);

            self.initialized = true;
        }
    }

    pub fn initialized(&self) -> bool {
        self.initialized
    }

    fn find_slot_with_item(
        &self,
        item: &(impl Component + Clone + BaseItem),
        slot_query: &Query<&mut InventorySlot>,
        item_query: &Query<&Item>,
    ) -> Option<SlotEntity> {
        for slot_entity in self.slots.iter() {
            if let Ok(slot) = slot_query.get(*slot_entity) {
                if slot.has_item(item, item_query) {
                    return Some(*slot_entity);
                }
            }
        }

        None
    }

    fn find_empty_slot(&self, slot_query: &Query<&mut InventorySlot>) -> Option<SlotEntity> {
        for slot_entity in self.slots.iter() {
            if let Ok(slot) = slot_query.get(*slot_entity) {
                if slot.item().is_some() {
                    continue;
                }

                return Some(*slot_entity);
            }
        }

        None
    }

    pub fn add_item(
        &mut self,
        commands: &mut Commands,
        item_data: &ItemRecord,
        slot_query: &mut Query<&mut InventorySlot>,
        item_query: &Query<&Item>,
        amount: u32,
    ) {
        // If we already have the item in the inventory, then we will just add the amount
        if item_data.stackable() {
            let item = item_data.data.clone();
            if let Some(slot_entity) = self.find_slot_with_item(&item, slot_query, item_query) {
                let Ok(mut slot) = slot_query.get_mut(slot_entity) else {
                    return;
                };

                slot.add_amount(amount);
                return;
            }
        }

        // Finds the first empty slot and adds the item
        if let Some(slot_entity) = self.find_empty_slot(slot_query) {
            let item = item_data.data.clone();
            let Ok(mut slot) = slot_query.get_mut(slot_entity) else {
                return;
            };

            // Creates a new item on the inventory slot
            let item_entity = commands.spawn(item).id();

            commands.entity(slot_entity).push_children(&[item_entity]);
            slot.update_slot(Some(item_entity), amount);
            return;
        }
    }
}

#[derive(Component, InspectorOptions, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct InventorySlot {
    pub item: Option<ItemEntity>,
    pub amount: u32,
}

impl ItemSlot for InventorySlot {
    fn amount(&self) -> u32 {
        self.amount
    }

    fn mut_amount(&mut self) -> &mut u32 {
        self.amount.as_mut()
    }

    fn item(&self) -> &Option<ItemEntity> {
        &self.item
    }

    fn mut_item(&mut self) -> &mut Option<ItemEntity> {
        &mut self.item
    }
}

#[derive(Component, InspectorOptions, Reflect, Default)]
#[reflect(Component)]
pub struct HotbarSlot {
    pub item: Option<ItemEntity>,
    pub amount: u32,
}

impl ItemSlot for HotbarSlot {
    fn amount(&self) -> u32 {
        self.amount
    }

    fn mut_amount(&mut self) -> &mut u32 {
        self.amount.as_mut()
    }

    fn item(&self) -> &Option<ItemEntity> {
        &self.item
    }

    fn mut_item(&mut self) -> &mut Option<ItemEntity> {
        &mut self.item
    }
}
