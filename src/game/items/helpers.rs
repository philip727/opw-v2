use bevy::{asset::Handle, reflect::Reflect, render::texture::Image};
use bevy_inspector_egui::InspectorOptions;
use serde::{Deserialize, Serialize};

use super::components::Item;

#[derive(Serialize, Deserialize, Debug, InspectorOptions, Reflect, Default)]
pub struct ItemRecord {
    pub data: Item,
    pub asset: String,
    pub properties: Vec<ItemProperties>,
}

impl ItemRecord {
    pub fn is_tool(&self) -> bool {
        self.properties.contains(&ItemProperties::Tool)
    }

    pub fn stackable(&self) -> bool {
        self.properties.contains(&ItemProperties::Stackable)
    }

    pub fn placeable(&self) -> bool {
        self.properties.contains(&ItemProperties::Placeable)
    }
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Reflect)]
pub enum ItemProperties {
    Tool,
    Stackable,
    Placeable,
}

pub trait BaseItem {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
}
