use bevy::prelude::*;

use crate::game::world::{
    helpers::ThresholdPos, ruletile::helpers::RuletileMap, textures::helpers::TextureMap,
};

#[derive(Debug, Event, Clone)]
pub struct RequestTextureMap {
    pub threshold_pos: ThresholdPos,
}

#[derive(Debug, Event, Clone)]
pub struct RequestChunkRender {
    pub ruletile_map: RuletileMap,
    pub texture_map: TextureMap,
    pub world_position: Vec3,
}
