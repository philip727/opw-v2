use std::{
    fmt::format,
    fs::{self, File},
    io::Write,
};

use serde::{Deserialize, Serialize};

use super::constants::{BIOMES_DATA_LOCATION, BIOMES_DATA_DEFAULT};

#[derive(Serialize, Deserialize, Debug)]
pub struct BiomeData {
    pub id: String,
    pub texture_location: String,
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
