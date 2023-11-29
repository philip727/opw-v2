use belly::core::relations::props::PropertyProtocol;
use bevy::{prelude::*, utils::HashMap};
use bevy_inspector_egui::prelude::*;


use super::helpers::ItemSlot;

pub type ItemEntity = Entity;
pub type SlotEntity = Entity;

#[derive(Component, InspectorOptions)]
pub struct Inventory {
    pub slots: HashMap<Entity, Option<SlotEntity>>,
}

impl Inventory {
    pub fn new() -> Self {
        Inventory {
            slots: HashMap::new(),
        }
    }

   // pub fn find_item_slot(&self, item: &dyn Item) -> Option<Entity> {
   //     self.slots.iter().find_map(|(entity, i)| {
   //         if i.as_ref().is_some_and(|i| item.id() == i.id()) {
   //             Some(*entity)
   //         } else {
   //             None
   //         }
   //     })
   // }
}

#[derive(Component, InspectorOptions)]
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

#[derive(Component, InspectorOptions)]
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
