use std::fs::{self, File};
use std::io::Write;

use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{TilePos, TileStorage, TileTextureIndex};

use crate::game::world::textures::constants::BIOMES_DATA_DEFAULT;
use crate::game::world::{
    biomes::helpers::BiomeData,
    generation::{components::Chunk, events::RequestChunkRender},
    resources::WorldManager,
};

use super::constants::BIOMES_DATA_LOCATION;

pub fn pack_textures() {
    let mut data = fs::read_to_string(BIOMES_DATA_LOCATION);
    // Creates the file if it doesn't exist and writes to it
    if let Err(..) = data {
        let mut file =
            File::create(BIOMES_DATA_LOCATION).expect("Failed to create biomes/data.json file.");

        file.write_all(BIOMES_DATA_DEFAULT.as_bytes())
            .expect("Failed to write to biomes/data.json file.");

        data = fs::read_to_string(BIOMES_DATA_LOCATION);
    }

    // Loads the json biome data
    let json = serde_json::from_str::<Vec<BiomeData>>(&data.unwrap());
    dbg!(json);
}

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
