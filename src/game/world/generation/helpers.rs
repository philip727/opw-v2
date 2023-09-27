use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use super::constants::{CHUNK_SIZE, CHUNK_Z_POS, TILE_SIZE};

pub trait IntoChunkPos {
    fn to_chunk_pos(self) -> ChunkPos;
}

pub trait IntoTranslation {
    fn to_translation(self) -> Vec3;
}

pub trait SetZToChunkZ<T> {
    fn set_z_to_chunk_z(self) -> T;
}

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub struct WorldPos {
    pub x: f32,
    pub y: f32,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
pub struct ChunkPos {
    pub x: i32,
    pub y: i32,
}

impl SetZToChunkZ<Vec3> for Vec3 {
    fn set_z_to_chunk_z(self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: CHUNK_Z_POS,
        }
    }
}

impl IntoTranslation for ChunkPos {
    fn to_translation(self) -> Vec3 {
        Vec3 {
            x: self.x as f32 * CHUNK_SIZE as f32 * TILE_SIZE,
            y: self.y as f32 * CHUNK_SIZE as f32 * TILE_SIZE,
            z: 0.0,
        }
    }
}

impl IntoChunkPos for Vec2 {
    fn to_chunk_pos(self) -> ChunkPos {
        let pos = self.as_ivec2();
        let chunk_size = IVec2::new(CHUNK_SIZE as i32, CHUNK_SIZE as i32);
        let tile_size = IVec2::new(TILE_SIZE as i32, TILE_SIZE as i32);
        let chunk_pos = pos / (chunk_size * tile_size);
        ChunkPos {
            x: chunk_pos.x,
            y: chunk_pos.x,
        }
    }
}

pub fn create_chunk_tilemap(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    chunk_pos: &ChunkPos,
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
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(0),
                    ..Default::default()
                })
                .id();

            commands.entity(tilemap_entity).add_child(tile_entity);
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let chunk_translation = chunk_pos.to_translation().set_z_to_chunk_z();
    let chunk_transform = Transform::from_translation(chunk_translation);

    let tile_size = TilemapTileSize {
        x: TILE_SIZE,
        y: TILE_SIZE,
    };

    let grid_size: TilemapGridSize = tile_size.into();
    let map_type = TilemapType::Square;

    let texture_handle: Handle<Image> = asset_server.load("tilemaps/temp.png");
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
        ))
        .id()
}
