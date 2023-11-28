use std::usize;

use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

use crate::game::items::helpers::Item;

#[derive(Component, InspectorOptions)]
pub struct Inventory {
    pub slots: Vec<Option<(Entity, u32)>>,
}

impl Inventory {
    pub fn with_capacity(size: usize) -> anyhow::Result<Inventory> {
        let inventory = Inventory {
            slots: Vec::with_capacity(size),
        };

        Ok(inventory)
    }
}

#[derive(Component, InspectorOptions)]
pub struct InventorySlot {
    pub item: Option<Item>,
    pub amount: u32,
}

#[derive(Component, InspectorOptions)]
pub struct HotbarSlot {
    pub item: Option<Item>,
    pub amount: u32,
}
