use std::{
    fs,
    path::{Path, PathBuf},
};

use bevy::{prelude::*, utils::HashMap};
use bevy_inspector_egui::InspectorOptions;

use super::{components::Item, errors::ItemError, helpers::ItemRecord};

pub type ItemId = u32;
#[derive(Resource, Reflect, InspectorOptions)]
#[reflect(Resource)]
pub struct ItemDatabase {
    pub items: HashMap<String, ItemRecord>,
}

impl Default for ItemDatabase {
    fn default() -> Self {
        let items = HashMap::new();
        ItemDatabase { items }
    }
}

impl ItemDatabase {
    // Load all items
    pub fn initialize(&mut self, path: PathBuf) {
        let items = ItemDatabase::load_items(path);

        for item in items.unwrap() {
            self.items.insert(item.data.id.clone(), item);
        }
    }

    pub fn get_item_data(&self, item: &Item) -> Option<&ItemRecord> {
        self.items.get(&item.id)
    }

    pub fn get_item_data_by_id(&self, id: &str) -> Option<&ItemRecord> {
        self.items.get(id)
    }

    pub fn load_items(path: PathBuf) -> anyhow::Result<Vec<ItemRecord>> {
        let json_path = path.to_str().unwrap().to_owned();
        let file_string = fs::read_to_string(&path).map_err(|_| ItemError::NoItemJson {
            path: json_path.clone(),
        })?;

        let items: Vec<ItemRecord> = serde_json::from_str::<Vec<ItemRecord>>(&file_string)
            .map_err(|e| ItemError::InvalidData {
                path: json_path.clone(),
                error: e.to_string(),
            })?;

        if items.len() < 1 {
            return Err(ItemError::NoItems {
                path: json_path.clone(),
            }.into());
        }

        Ok(items)
    }
}
