use bevy::{prelude::*, utils::HashMap};

use crate::game::world::biomes::helpers::TileId;

#[derive(Resource, Default)]
pub struct TileAnimationManager {
    pub synced_animations: HashMap<TileId, SyncedTileAnimation>,
}

impl TileAnimationManager {
    fn next(&mut self, tile_id: &TileId) {
        if let Some(synced_animation) = self.synced_animations.get_mut(tile_id) {
            if synced_animation.currrent_index >= synced_animation.texture_length - 1 {
                synced_animation.currrent_index = 0;
            } else {
                synced_animation.currrent_index += 1;
            }
        }
    }

    pub fn update_synced_tile(&mut self, tile_id: &TileId, add: f32) {
        if let Some(synced_animation) = self.synced_animations.get_mut(tile_id) {
            let time_between_frames = synced_animation.animation_length / synced_animation.texture_length as f32;

            synced_animation.current_time += add;
            if synced_animation.current_time > time_between_frames {
                synced_animation.current_time = 0.0;
                self.next(tile_id);
            }
        }
    }
}

#[derive(Debug)]
pub struct SyncedTileAnimation {
    pub currrent_index: usize,
    pub texture_length: usize,
    pub current_time: f32,
    pub animation_length: f32,
}
