use bevy::{prelude::*, utils::HashMap};

use super::helpers::{BiomeData, BiomeId, BiomeOffset};

#[derive(Debug, Resource, Default, Clone)]
pub struct BiomeManager {
    pub loaded: Vec<BiomeData>,
    pub offsets: HashMap<BiomeId, BiomeOffset>,
}

impl BiomeManager {
    pub fn get_biome_offset(&self, biome: &BiomeData) -> Option<&BiomeOffset> {
        self.offsets.get(&biome.id)
    }
}
