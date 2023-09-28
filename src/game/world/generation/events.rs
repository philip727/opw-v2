use bevy::prelude::*;

use crate::game::world::{helpers::WorldPos, textures::helpers::TextureMap};

#[derive(Debug, Event, Clone)]
pub struct RequestTextureMap {
    pub world_position: WorldPos
}

#[derive(Debug, Event, Clone)]
pub struct RequestChunkRender {
    pub texture_map: TextureMap,
    pub world_position: WorldPos
}
