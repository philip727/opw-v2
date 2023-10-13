use std::{
    fs::{self, File},
    io::Write,
};

use bevy::prelude::info;
use serde::{Deserialize, Serialize};

use crate::math::noise::{euclidian_distance, normalize_noise_value};

use super::{
    constants::{BIOMES_DATA_DEFAULT, BIOMES_DATA_LOCATION},
    resources::BiomeManager,
};

pub type BiomeId = String;
pub type BiomeOffset = u32;

/// The data of a biome
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BiomeData {
    pub id: BiomeId,
    pub texture_location: String,
    pub rules: BiomeRules,
    pub tiles: BiomeTiles,
}

/// The rules to spawn a biome for generation
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BiomeRules {
    pub precipitation: f32,
    pub temperature: f32,
}

/// The common tiles on the tilemap
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BiomeTiles {
    pub water: TileTextureData,
    pub top_left: TileTextureData,
    pub top_middle: TileTextureData,
    pub top_right: TileTextureData,
    pub middle: TileTextureData,
    pub middle_left: TileTextureData,
    pub middle_right: TileTextureData,
    pub down_right: TileTextureData,
    pub down_left: TileTextureData,
    pub up_right: TileTextureData,
    pub up_left: TileTextureData,
    pub bottom_left: TileTextureData,
    pub bottom_middle: TileTextureData,
    pub bottom_right: TileTextureData,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TileTextureData {
    pub style: TileStyle,
    pub textures: Vec<u8>,
    pub animation_length: f32,
    index: usize,

}

impl TileTextureData {
    pub fn new() -> TileTextureData {
        TileTextureData {
            style: TileStyle::Single,
            textures: Vec::new(),
            animation_length: 1.0f32,
            index: 0,
        }
    }

    pub fn get_offset(&self) -> u8 {
        match self.style {
            TileStyle::Single => self.textures[0],
            TileStyle::Animated => self.textures[0],
            TileStyle::Random => self.textures[0],
        }
    }
}

impl Iterator for TileTextureData {
    type Item = u8;

    // Iterates over each tile texture
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.textures[self.index];
        info!("{}", item);
        if self.index >= self.textures.len() - 1 {
            self.index = 0;
            return None;
        } else {
            self.index += 1
        };

        Some(item)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum TileStyle {
    Single,
    Animated,
    Random,
}

pub fn load_biome_data() -> Vec<BiomeData> {
    let mut data = fs::read_to_string(BIOMES_DATA_LOCATION);
    // Creates the file if it doesn't exist and writes to it
    if let Err(..) = data {
        let mut file = File::create(BIOMES_DATA_LOCATION)
            .expect(&format!("Failed to create {} file.", BIOMES_DATA_LOCATION));

        file.write_all(BIOMES_DATA_DEFAULT.as_bytes())
            .expect(&format!(
                "Failed to write to {} file.",
                BIOMES_DATA_LOCATION
            ));

        data = fs::read_to_string(BIOMES_DATA_LOCATION);
    }

    // Loads the json biome data
    serde_json::from_str::<Vec<BiomeData>>(&data.unwrap())
        .expect(&format!("Failed to load {} json.", BIOMES_DATA_LOCATION))
}

pub fn determine_best_biome<'manager>(
    precipitation: f32,
    temperature: f32,
    biome_manager: &'manager BiomeManager,
) -> &'manager BiomeData {
    let mut best_biome: &BiomeData = &biome_manager.loaded[0];
    let mut best_euclidian = 999999f32;
    for biome in biome_manager.loaded.iter() {
        let euclidian = euclidian_distance(
            normalize_noise_value(temperature),
            normalize_noise_value(precipitation),
            biome.rules.temperature,
            biome.rules.precipitation,
        );

        if euclidian < best_euclidian {
            best_biome = &biome;
            best_euclidian = euclidian;
        }
    }

    best_biome
}
