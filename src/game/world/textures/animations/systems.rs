use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TileTextureIndex;

use crate::game::world::{biomes::helpers::TileStyle, collisions::components::TileProperties};

pub fn handle_tile_animations(
    mut tile_query: Query<(&mut TileProperties, &mut TileTextureIndex)>,
    time: Res<Time>,
) {
    for (mut properties, mut texture_index) in tile_query.iter_mut() {
        if properties.data.style != TileStyle::Animated {
            continue;
        }

        let time_between_frames =
            properties.data.animation_length / properties.data.textures.len() as f32;

        if properties.update_animation_time(time.delta_seconds(), time_between_frames) {
            if let Some(new_index) = properties.data.next() {
                *texture_index = TileTextureIndex(new_index as u32);
            }
        }
    }
}
