use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use noise::utils::NoiseMap;

use crate::game::world::{
    collisions::components::TileProperties,
    helpers::{
        adjust_translation_for_chunk, ChunkPos, IntoTranslation, SetZToChunkZ, ThresholdPos,
    },
    textures::constants::ASSET_TEXTURE_ATLAS_PATH,
};

use super::{
    components::Chunk,
    constants::{CHUNK_SIZE, TILE_SIZE},
};

#[repr(transparent)]
pub struct HeightNoiseMap(pub NoiseMap);

#[repr(transparent)]
pub struct TemperatureNoiseMap(pub NoiseMap);

#[repr(transparent)]
pub struct PrecipitationNoiseMap(pub NoiseMap);

// Creates the tilemap that the chunk uses
pub fn create_chunk_tilemap(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    threshold_pos: &ThresholdPos,
) -> Entity {
    let tilemap_entity = commands.spawn_empty().id();
    let tilemap_size = TilemapSize {
        x: CHUNK_SIZE,
        y: CHUNK_SIZE,
    };

    let mut tile_storage = TileStorage::empty(tilemap_size);
    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn((
                    TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex(0),
                        ..Default::default()
                    },
                    TileProperties::default(),
                ))
                .id();

            commands.entity(tilemap_entity).add_child(tile_entity);
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    // Place the chunk at the center rather than from the bottom left
    let chunk_translation =
        adjust_translation_for_chunk(threshold_pos.to_translation().set_z_to_chunk_z());

    let chunk_transform = Transform::from_translation(chunk_translation);
    let tile_size = TilemapTileSize {
        x: TILE_SIZE,
        y: TILE_SIZE,
    };

    let grid_size: TilemapGridSize = tile_size.into();
    let map_type = TilemapType::Square;

    let texture_handle: Handle<Image> = asset_server.load(ASSET_TEXTURE_ATLAS_PATH);
    let tilemap_texture = TilemapTexture::Single(texture_handle);

    commands
        .entity(tilemap_entity)
        .insert((
            Name::new("Main Chunk"),
            TilemapBundle {
                grid_size,
                map_type,
                size: tilemap_size,
                storage: tile_storage,
                texture: tilemap_texture,
                tile_size,
                transform: chunk_transform,
                ..Default::default()
            },
            Chunk,
        ))
        .id()
}
