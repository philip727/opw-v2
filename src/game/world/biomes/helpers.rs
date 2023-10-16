use std::{
    fs::{self},
    path::{Path, PathBuf},
};

use bevy::prelude::info;
use serde::{Deserialize, Serialize};

use crate::math::noise::{euclidian_distance, normalize_noise_value};

use super::{
    constants::BIOMES_DIR_PATH,
    errors::BiomeError,
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

pub type TileId = String;
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TileTextureData {
    pub id: TileId,
    pub sync_group_id: Option<String>,
    pub style: TileStyle,
    pub frames: Vec<u8>,
    pub animation_length: f32,
}

impl TileTextureData {
    pub fn new() -> TileTextureData {
        TileTextureData {
            id: String::new(),
            sync_group_id: None,
            style: TileStyle::Single,
            frames: Vec::new(),
            animation_length: 1.0f32,
        }
    }

    pub fn get_first_frame(&self) -> u8 {
        self.frames[0]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum TileStyle {
    Single,
    Animated,
    Random,
}

impl BiomeData {
    /// Loads all the biomes from [path]
    pub fn load_biomes<P: AsRef<Path>>(path: P) -> anyhow::Result<Vec<Self>> {
        let mut biomes: Vec<_> = vec![];

        if let Ok(biome_dir) = fs::read_dir(path) {
            for biome_dir in biome_dir {
                if let Ok(entry) = biome_dir {
                    let biome_data = Self::load_biome_data(entry.path());
                    match biome_data {
                        Ok(biome_data) => biomes.push(biome_data),
                        Err(e) => {
                            info!("{}", e)
                        }
                    }
                }
            }
        }

        if biomes.len() < 1 {
            return Err(BiomeError::NoBiomes {
                path: BIOMES_DIR_PATH.to_string(),
            }
            .into());
        }

        Ok(biomes)
    }

    /// Loads the biome data from a directory. [path] must be a directory.
    fn load_biome_data(path: PathBuf) -> anyhow::Result<BiomeData> {
        let biome_data_path = path.to_str().unwrap().to_owned() + "/data.json";
        let biome_dir_name = path.file_name().unwrap().to_str().unwrap().to_owned();

        // Turns the file contents into a string
        let file_string = fs::read_to_string(&biome_data_path).map_err(|e| BiomeError::NoData {
            name: biome_dir_name.clone(),
            error: e.to_string(),
        })?;

        // Serializes string into biome data
        let biome_data = serde_json::from_str::<BiomeData>(&file_string).map_err(|e| {
            BiomeError::InvalidData {
                name: biome_dir_name.clone(),
                error: e.to_string(),
            }
        })?;

        Ok(biome_data)
    }
}

/// Finds the best biome using euclidian
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
