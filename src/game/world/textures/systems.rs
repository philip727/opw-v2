use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{TilePos, TileStorage, TileTextureIndex};

use crate::game::world::biomes::resources::BiomeManager;
use crate::game::world::collisions::components::TileProperties;
use crate::game::world::helpers::adjust_translation_for_chunk;
use crate::game::world::states::WorldState;
use crate::game::world::textures::helpers::generate_texture_atlas;
use crate::game::world::{
    generation::{components::Chunk, events::RequestChunkRender},
    resources::WorldManager,
};
use crate::math::map::ValueMap2D;

pub fn pack_textures(biome_manager: Res<BiomeManager>, mut commands: Commands) {
    generate_texture_atlas(&biome_manager.loaded);

    info!("Texture atlas created");
    commands.insert_resource(NextState(Some(WorldState::Created)));
}

pub fn handle_chunk_rerender(
    mut request_chunk_rerender_reader: EventReader<RequestChunkRender>,
    world_manager: Res<WorldManager>,
    mut chunk_query: Query<(&mut TileStorage, &mut Transform), With<Chunk>>,
    mut tile_query: Query<(&TilePos, &mut TileTextureIndex, &mut TileProperties)>,
) {
    for event in request_chunk_rerender_reader.iter() {
        if let Some(chunk_entity) = world_manager.chunk_entity {
            if let Ok((tile_storage, mut transform)) = chunk_query.get_mut(chunk_entity) {
                let texture_map = &event.texture_map;
                let translation = adjust_translation_for_chunk(event.world_position);

                // Updates all the tiles in the tile storage
                for tile_entity in tile_storage.iter() {
                    let tile_entity = tile_entity.unwrap();

                    let (tile_pos, mut tile_texture_index, mut tile_properties) =
                        tile_query.get_mut(tile_entity).unwrap();

                    let texture_index =
                        texture_map.get_value(tile_pos.x as usize, tile_pos.y as usize);

                    if let Some((biome_offset, data)) = texture_index {
                        // Clones the data so we can animate and do things with tiles
                        tile_properties.data = data.clone();
                        tile_properties.biome_offset = biome_offset;

                        *tile_texture_index =
                            TileTextureIndex(biome_offset + data.get_first_frame() as u32);
                    }
                }

                // Update the position of the chunk
                transform.translation = translation;
            }
        }
    }
}
