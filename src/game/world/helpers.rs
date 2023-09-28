use bevy::prelude::*;

use super::generation::constants::{CHUNK_SIZE, CHUNK_Z_POS, TILE_SIZE};

pub trait IntoChunkPos {
    fn to_chunk_pos(self) -> ChunkPos;
}

pub trait IntoTranslation {
    fn to_translation(self) -> Vec3;
}

pub trait IntoWorldPos {
    fn to_world_pos(self) -> WorldPos;
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

impl IntoWorldPos for Vec3 {
    fn to_world_pos(self) -> WorldPos {
        WorldPos {
            x: self.x / TILE_SIZE,
            y: self.y / TILE_SIZE,
        }
    }
}
