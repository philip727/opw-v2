use bevy::prelude::*;
use rand::rngs::StdRng;

use crate::game::world::helpers::ThresholdPos;

#[derive(Resource)]
pub struct WorldGenerationManager {
    pub last_update_pos: ThresholdPos,
    pub rng: Option<StdRng>,
    pub seed: u32,
}

impl Default for WorldGenerationManager {
    fn default() -> Self {
        Self {
            last_update_pos: ThresholdPos { x: 0, y: 0 },
            rng: None,
            seed: 0,
        }
    }
}
