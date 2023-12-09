use bevy::reflect::Reflect;
use bevy_inspector_egui::InspectorOptions;
use serde::{Serialize, Deserialize};

use super::components::Item;

#[derive(Serialize, Deserialize, Debug, InspectorOptions, Reflect, Default)]
pub struct ItemRecord {
    pub data: Item,
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
