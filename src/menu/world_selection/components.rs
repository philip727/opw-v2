use std::{path::{Path, PathBuf}, fs};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::errors::WorldSelectionError;

#[derive(Component, Default)]
pub struct WorldSelectionRoot;

#[derive(Component)]
pub struct WorldsContainer;

#[derive(Serialize, Deserialize, Component, Clone, Debug, PartialEq, Copy)]
pub struct WorldSelectionData {
    pub seed: u32,
}

impl WorldSelectionData {
    pub fn load_all<P: AsRef<Path>>(path: P) -> anyhow::Result<Vec<(String, Self)>> {
        let mut worlds: Vec<_> = vec![];

        if let Ok(worlds_dir) = fs::read_dir(path) {
            for world_dir in worlds_dir {
                if let Ok(entry) = world_dir {
                    let world_data = WorldSelectionData::load(entry.path());
                    match world_data {
                        Ok(world_data) => worlds.push(world_data),
                        Err(e) => {
                            info!("{}", e);
                        }
                    }
                }
            }
        }

        if worlds.len() < 1 {
            return Err(WorldSelectionError::NoneFound.into());
        }

        Ok(worlds)
    }

    pub fn load(path: PathBuf) -> anyhow::Result<(String, WorldSelectionData)> {
        #![allow(deprecated)]
        use base64::decode;

        let world_data_path = path.to_str().unwrap().to_owned() + "/settings.data";
        let world_name = path.file_name().unwrap().to_str().unwrap().to_owned();

        let file_string =
            fs::read_to_string(&world_data_path).map_err(|_| WorldSelectionError::NoSettings {
                name: world_name.clone(),
            })?;

        let decoded_string =
            decode(file_string.as_bytes()).map_err(|e| WorldSelectionError::FailedDecode {
                name: world_name.clone(),
                error: e.to_string(),
            })?;

        let world_data =
            serde_json::from_str::<WorldSelectionData>(&String::from_utf8(decoded_string).unwrap())
                .map_err(|e| WorldSelectionError::InvalidData {
                    name: world_name.clone(),
                    error: e.to_string(),
                })?;

        Ok((world_name, world_data))
    }
}
