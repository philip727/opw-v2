use std::{
    fs::{self, File},
    io::Write,
};

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
    pub water: u8,
    pub top_left: u8,
    pub top_middle: u8,
    pub top_right: u8,
    pub middle: u8,
    pub middle_left: u8,
    pub middle_right: u8,
    pub down_right: u8,
    pub down_left: u8,
    pub up_right: u8,
    pub up_left: u8,
    pub bottom_left: u8,
    pub bottom_middle: u8,
    pub bottom_right: u8,
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
