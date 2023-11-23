use bevy::prelude::Resource;

use super::components::WorldSelectionData;

#[derive(Resource, Default)]
pub struct WorldSelectionManager {
    pub selected_world: Option<WorldSelectionData>
}
