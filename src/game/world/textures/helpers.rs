use bevy::{prelude::UVec2, utils::HashMap};

use crate::game::world::generation::{
    constants::{CHUNK_SIZE, WATER_HEIGHT_THRESHOLD, WATER_PRECIPITATION_THRESHOLD},
    helpers::*,
};

#[derive(Clone, Debug)]
pub struct TextureMap {
    pub size: (usize, usize),
    points: Vec<u32>,
}

impl TextureMap {
    pub fn new(size: (usize, usize)) -> Self {
        let (width, height) = size;
        let map_size = width * height;
        let mut texture_map = TextureMap {
            size,
            points: vec![0; map_size],
        };

        texture_map
    }

    pub fn set_value(&mut self, x: usize, y: usize, value: u32) {
        let (width, height) = self.size;

        if x < width && y < height {
            self.points[x + y * width] = value;
        }
    }

    pub fn get_value(&self, x: usize, y: usize) -> Option<u32> {
        let (width, height) = self.size;

        if x < width && y < height {
            return Some(self.points[x + y * width]);
        }

        None
    }
}

// Creates a texture map for us to render the chunk
pub fn create_texture_map(
    height_noise_map: HeightNoiseMap,
    temperature_noise_map: TemperatureNoiseMap,
    precipiation_noise_map: PrecipitationNoiseMap,
) -> TextureMap {
    let mut texture_map = TextureMap::new((CHUNK_SIZE as usize, CHUNK_SIZE as usize));

    // Determines what tiles to put at certain co-ordinates
    for x in 0..CHUNK_SIZE as usize {
        for y in 0..CHUNK_SIZE as usize {
            let height = height_noise_map.0.get_value(x, y);
            let precipiation = precipiation_noise_map.0.get_value(x, y);

            if is_water_tile(height, precipiation) {
                texture_map.set_value(x, y, 0);
                continue;
            }

            texture_map.set_value(x, y, 1);
        }
    }

    texture_map
}

pub fn is_water_tile(height: f64, precipiation: f64) -> bool {
    height < WATER_HEIGHT_THRESHOLD && precipiation > WATER_PRECIPITATION_THRESHOLD
}
