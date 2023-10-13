use bevy::prelude::*;

use crate::game::world::helpers::ThresholdPos;

#[derive(Resource)]
pub struct WorldGenerationManager {
    pub last_update_pos: ThresholdPos,
}

impl Default for WorldGenerationManager {
    fn default() -> Self {
        Self {
            last_update_pos: ThresholdPos { x: 0, y: 0 },
        }
    }
}
