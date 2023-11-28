use bevy::prelude::Resource;

use super::components::WorldSelectionData;

#[derive(Resource, Default, Debug)]
pub struct WorldSelectionManager {
    pub name: Option<String>,
    pub selected_world: Option<WorldSelectionData>,
}

impl WorldSelectionManager {
    pub fn get_world_name(&self) -> String {
        if self.name.is_none() {
            return "No world".into();
        }

        return self.name.as_ref().unwrap().to_string();
    }
}
