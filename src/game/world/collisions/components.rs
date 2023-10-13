use bevy::prelude::*;

use crate::game::world::biomes::helpers::TileTextureData;

#[derive(Component, Debug)]
pub struct TileProperties {
    pub collidable: bool,
    pub data: TileTextureData,
    last_frame_time: f32,
}

impl TileProperties {
    pub fn update_animation_time(&mut self, add: f32, time_between_frame: f32) -> bool {
        self.last_frame_time += add;

        if self.last_frame_time > time_between_frame {
            self.last_frame_time = 0.0;
            return true;
        }

        false
    }
}

impl Default for TileProperties {
    fn default() -> Self {
        TileProperties {
            collidable: false,
            data: TileTextureData::new(),
            last_frame_time: 0.0f32,
        }
    }
}
