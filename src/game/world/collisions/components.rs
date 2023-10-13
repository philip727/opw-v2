use bevy::prelude::*;

use crate::game::world::biomes::helpers::TileTextureData;

#[derive(Component, Debug)]
pub struct TileProperties {
    pub collidable: bool,
    pub data: TileTextureData,
    pub biome_offset: u32,
}

impl Default for TileProperties {
    fn default() -> Self {
        TileProperties {
            collidable: false,
            data: TileTextureData::new(),
            biome_offset: 0,
        }
    }
}
