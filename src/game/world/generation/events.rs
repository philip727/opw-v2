use bevy::prelude::*;

use crate::game::world::{helpers::{WorldPos, ThresholdPos}, textures::helpers::TextureMap};

#[derive(Debug, Event, Clone)]
pub struct RequestTextureMap {
    pub threshold_pos: ThresholdPos
}

#[derive(Debug, Event, Clone)]
pub struct RequestChunkRender {
    pub texture_map: TextureMap,
    pub world_position: Vec3
}
