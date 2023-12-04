use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;
use serde::{Deserialize, Serialize};

use super::helpers::BaseItem;

#[derive(Component, InspectorOptions, Reflect, Default, Clone, Debug, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Item {
    pub id: String,
    pub name: String,
}

impl BaseItem for Item {
    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }
}
