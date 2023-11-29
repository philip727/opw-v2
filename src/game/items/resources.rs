use bevy::{utils::HashMap, prelude::*};

use super::helpers::ItemData;

pub type ItemId = u32;
#[derive(Resource, Default)]
pub struct ItemDatabase {
    pub items: HashMap<ItemId, ItemData>
}

impl ItemDatabase {
    // Load all items
    pub fn initialize(&mut self) {
        todo!()
    }
}
