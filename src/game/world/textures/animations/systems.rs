use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TileTextureIndex;

use crate::game::world::{
    biomes::helpers::{BiomeId, TileStyle},
    collisions::components::TileProperties,
};

use super::resources::{SyncedTileAnimation, TileAnimationManager};

// Handles syncing all the animations
pub fn handle_synced_animations(
    mut tile_query: Query<&mut TileProperties>,
    mut tile_animation_manager: ResMut<TileAnimationManager>,
    time: Res<Time>,
) {
    let mut handle_animations: Vec<BiomeId> = Vec::new();

    for properties in tile_query.iter_mut() {
        if properties.data.style != TileStyle::Animated {
            continue;
        }

        if let None = properties.data.sync_group_id {
            continue;
        }

        let sync_group_id = properties.data.sync_group_id.as_ref().unwrap();

        // If the tile hasn't been synced yet, then we must set it up
        if let None = tile_animation_manager.synced_animations.get(sync_group_id) {
            tile_animation_manager.synced_animations.insert(
                sync_group_id.clone(),
                SyncedTileAnimation {
                    currrent_index: 0,
                    texture_length: properties.data.textures.len(),
                    current_time: 0.0,
                    animation_length: properties.data.animation_length,
                },
            );
        }

        // Gets all the ids we need to handle at the moment
        if handle_animations.contains(sync_group_id) {
            continue;
        }

        handle_animations.push(sync_group_id.to_string());
    }

    // Updates all the synced animations
    for id in handle_animations {
        tile_animation_manager.update_synced_tile(&id, time.delta_seconds());
    }
}

// Sets the texture of each synced animation
pub fn handle_animated_tiles(
    mut tile_query: Query<(&TileProperties, &mut TileTextureIndex)>,
    tile_animation_manager: Res<TileAnimationManager>,
) {
    for (properties, mut texture_index) in tile_query.iter_mut() {
        if properties.data.style != TileStyle::Animated {
            continue;
        }

        if let None = properties.data.sync_group_id {
            continue;
        }

        if let Some(synced_animation) = tile_animation_manager
            .synced_animations
            .get(properties.data.sync_group_id.as_ref().unwrap())
        {
            *texture_index = TileTextureIndex(
                properties.biome_offset
                    + properties.data.textures[synced_animation.currrent_index] as u32,
            );
        }
    }
}
