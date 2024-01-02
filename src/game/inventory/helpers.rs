use bevy::prelude::*;

use crate::game::items::{components::Item, helpers::BaseItem};

pub type InventoryEntity = Entity;

pub trait ItemSlot {
    fn amount(&self) -> u32;

    fn mut_amount(&mut self) -> &mut u32;

    fn set_amount(&mut self, amount: u32) {
        *self.mut_amount() = amount;
    }

    fn add_amount(&mut self, amount: u32) {
        self.update_slot(*self.item(), self.amount() + amount);
    }

    fn item(&self) -> &Option<Entity>;

    fn mut_item(&mut self) -> &mut Option<Entity>;

    fn set_item(&mut self, item: Option<Entity>) {
        *self.mut_item() = item;
    }

    fn update_slot(&mut self, item: Option<Entity>, amount: u32) {
        self.set_item(item);
        self.set_amount(amount);
    }

    fn has_item(&self, item: &(impl Component + BaseItem), item_query: &Query<&Item>) -> bool {
        if let Some(entity) = self.item() {
            let Ok(found_item) = item_query.get(*entity) else {
                return false;
            };

            if item.id() == found_item.id {
                return true;
            }
        }

        false
    }
}
