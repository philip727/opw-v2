use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{TilePos, TileStorage, TileTextureIndex};

use crate::game::world::{
    generation::{components::Chunk, events::RequestChunkRender},
    resources::WorldManager,
};

pub fn pack_textures() {}

pub fn handle_chunk_rerender(
    mut request_chunk_rerender_reader: EventReader<RequestChunkRender>,
    mut world_manager: ResMut<WorldManager>,
    mut chunk_query: Query<&mut TileStorage, With<Chunk>>,
    mut tile_query: Query<(&TilePos, &mut TileTextureIndex)>,
) {
    for event in request_chunk_rerender_reader.iter() {
        let chunk_entity = world_manager.chunk_entity.unwrap();
        let texture_map = &event.texture_map;

        if let Ok(mut tile_storage) = chunk_query.get_mut(chunk_entity) {
            for tile_entity in tile_storage.iter() {
                let tile_entity = tile_entity.unwrap();

                let (tile_pos, mut tile_texture_index) = tile_query.get_mut(tile_entity).unwrap();
                let texture_index = texture_map.get_value(tile_pos.x as usize, tile_pos.y as usize);

                if let Some(texture_index) = texture_index {
                    *tile_texture_index = TileTextureIndex(texture_index);
                }
            }
        }
    }
}
