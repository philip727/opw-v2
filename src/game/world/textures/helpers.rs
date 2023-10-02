use bevy::prelude::info;
use image::{io::Reader as ImageReader, GenericImageView, ImageBuffer};

use crate::game::world::{
    biomes::helpers::BiomeData,
    generation::{
        constants::{CHUNK_SIZE, TILE_SIZE, WATER_HEIGHT_THRESHOLD, WATER_PRECIPITATION_THRESHOLD},
        helpers::*,
    },
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

pub fn generate_packed_texture_map(biome_data: &Vec<BiomeData>) {
    let tile_size_u32 = TILE_SIZE as u32;

    // Image data
    const IMAGE_WIDTH: u32 = 1024;
    let tiles_per_row_packed = IMAGE_WIDTH / tile_size_u32;
    let mut current_col_pos_packed = 0u32;
    let mut image_height: u32 = tile_size_u32;

    // Finds the total width and height for the texture map
    for biome in biome_data.iter() {
        let image = ImageReader::open(&biome.texture_location)
            .expect(&format!("Failed to load \"{}\"", biome.texture_location))
            .decode()
            .expect(&format!("Failed to load \"{}\"", biome.texture_location));

        let tiles_in_image = (image.width() / tile_size_u32) * (image.height() / tile_size_u32);
        current_col_pos_packed += tiles_in_image;

        if current_col_pos_packed > tiles_per_row_packed {
            image_height += tile_size_u32;
            current_col_pos_packed = current_col_pos_packed - tiles_per_row_packed;
        }
    }

    let packed_map = create_packed_map(IMAGE_WIDTH, image_height, tile_size_u32, &biome_data);
    packed_map
        .save("assets/tilemaps/packed_tilemap.png")
        .expect("Failed to save packed tilemap");
}

pub fn is_water_tile(height: f64, precipiation: f64) -> bool {
    height < WATER_HEIGHT_THRESHOLD && precipiation > WATER_PRECIPITATION_THRESHOLD
}

// Create packed map image buf
fn create_packed_map(
    image_width: u32,
    image_height: u32,
    tile_size: u32,
    biome_data: &Vec<BiomeData>,
) -> ImageBuffer<image::Rgba<u8>, Vec<u8>> {
    let tiles_per_row_packed = image_width / tile_size;

    let mut packed_map: ImageBuffer<image::Rgba<u8>, Vec<u8>> =
        ImageBuffer::new(image_width, image_height);

    let mut col_pos_packed = 0u32;
    let mut row_pos_packed = 0u32;
    for biome in biome_data.iter() {
        let image = ImageReader::open(&biome.texture_location)
            .expect(&format!("Failed to load \"{}\"", biome.texture_location))
            .decode()
            .expect(&format!("Failed to load \"{}\"", biome.texture_location));

        let tiles_in_tilemap = ((image.width() / tile_size) * (image.height() / tile_size));
        let tiles_per_row_tilemap = image.width() / tile_size;

        let mut row_pos_tilemap = 0u32;
        for col_pos_tilemap in 0..tiles_in_tilemap {
            // Gets the current x, y of the tile in the tilemap
            let tile_pos_tilemap_x = col_pos_tilemap % tiles_per_row_tilemap;
            let tile_pos_packed_x = col_pos_packed % tiles_per_row_packed;

            if tile_pos_tilemap_x % tiles_per_row_tilemap == 0 && col_pos_tilemap != 0 {
                row_pos_tilemap += 1;
            }

            if tile_pos_packed_x % tiles_per_row_packed == 0 && col_pos_packed != 0 {
                row_pos_packed += 1;
            }

            info!(
                "(1) packed: ({}, {}) // tilemap: ({}, {})",
                tile_pos_packed_x, row_pos_packed, tile_pos_tilemap_x, row_pos_tilemap
            );

            let dest_x_tilemap = tile_pos_tilemap_x * tile_size;
            let dest_y_tilemap = row_pos_tilemap * tile_size;

            let dest_x_packed = tile_pos_packed_x * tile_size;
            let dest_y_packed = row_pos_packed * tile_size;

            for pixel_x in 0..tile_size {
                for pixel_y in 0..tile_size {
                    let src_pixel =
                        image.get_pixel(dest_x_tilemap + pixel_x, dest_y_tilemap + pixel_y);

                    packed_map.put_pixel(
                        dest_x_packed + pixel_x,
                        dest_y_packed + pixel_y,
                        src_pixel,
                    );
                }
            }

            col_pos_packed += 1;
        }
    }

    packed_map
}
