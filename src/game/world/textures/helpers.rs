use image::{io::Reader as ImageReader, GenericImageView, ImageBuffer};

use crate::{
    game::world::{
        biomes::{
            helpers::{determine_best_biome, BiomeData, BiomeOffset, TileTextureData},
            resources::BiomeManager,
        },
        generation::{
            constants::{
                CHUNK_SIZE, TILE_SIZE, WATER_HEIGHT_THRESHOLD, WATER_PRECIPITATION_THRESHOLD,
            },
            helpers::*,
        },
        ruletile::helpers::{RuleTile, RuletileMap},
        textures::constants::TEXTURE_ATLAS_PATH,
    },
    math::map::ValueMap2D,
};

use super::constants::{
    FILLED_HEIGHT, NEIGHBOURS_TO_CHECK, NON_FILLED_HEIGHT, REQUIRED_NEIGHBOURS,
};

#[derive(Clone, Debug)]
pub struct HeightMap {
    pub size: (usize, usize),
    points: Vec<u8>,
}

impl ValueMap2D<u8> for HeightMap {
    fn new(size: (usize, usize)) -> Self {
        let (width, height) = size;
        let map_size = width * height;
        let texture_map = Self {
            size,
            points: vec![0; map_size],
        };

        texture_map
    }

    fn get_size(&self) -> (usize, usize) {
        self.size
    }

    fn get_points(&self) -> &[u8] {
        self.points.as_slice()
    }

    fn mut_points(&mut self) -> &mut Vec<u8> {
        &mut self.points
    }
}

#[derive(Clone, Debug)]
pub struct TextureMap {
    pub size: (usize, usize),
    points: Vec<(BiomeOffset, TileTextureData)>,
}

impl TextureMap {
    pub fn generate(
        temperature_noise_map: &TemperatureNoiseMap,
        precipitation_noise_map: &PrecipitationNoiseMap,
        ruletile_map: &RuletileMap,
        biome_manager: &BiomeManager,
    ) -> TextureMap {
        let mut texture_map = TextureMap::new((CHUNK_SIZE as usize, CHUNK_SIZE as usize));
        for x in 0..CHUNK_SIZE as usize {
            for y in 0..CHUNK_SIZE as usize {
                let precipitation = precipitation_noise_map.0.get_value(x, y) as f32;
                let temperature = temperature_noise_map.0.get_value(x, y) as f32;
                let ruletile = ruletile_map.get_value(x, y).unwrap_or(RuleTile::Water);

                let biome = determine_best_biome(precipitation, temperature, biome_manager);
                let tile_offset = biome_manager.get_biome_offset(biome).unwrap();

                texture_map.set_value(x, y, (*tile_offset, ruletile.get_tile_data(&biome).clone()));
            }
        }

        texture_map
    }
}

impl ValueMap2D<(BiomeOffset, TileTextureData)> for TextureMap {
    fn new(size: (usize, usize)) -> Self {
        let (width, height) = size;
        let map_size = width * height;
        let texture_map = Self {
            size,
            points: vec![(0, TileTextureData::new()); map_size],
        };

        texture_map
    }

    fn get_size(&self) -> (usize, usize) {
        self.size
    }

    fn get_points(&self) -> &[(BiomeOffset, TileTextureData)] {
        self.points.as_slice()
    }

    fn mut_points(&mut self) -> &mut Vec<(BiomeOffset, TileTextureData)> {
        &mut self.points
    }
}

impl HeightMap {
    // Creates a height map for us to determine textures
    pub fn generate(
        height_noise_map: &HeightNoiseMap,
        precipitation_noise_map: &PrecipitationNoiseMap,
    ) -> HeightMap {
        let mut height_map = HeightMap::new((CHUNK_SIZE as usize, CHUNK_SIZE as usize));
        // Determines what tiles to put at certain co-ordinates
        for x in 0..CHUNK_SIZE as usize {
            for y in 0..CHUNK_SIZE as usize {
                let height = height_noise_map.0.get_value(x, y) as f32;
                let precipitation = precipitation_noise_map.0.get_value(x, y) as f32;

                if is_water_tile(height, precipitation) {
                    height_map.set_value(x, y, NON_FILLED_HEIGHT);
                    continue;
                }

                height_map.set_value(x, y, FILLED_HEIGHT);
            }
        }

        height_map
    }

    // Smooths the height map to show only
    pub fn smoothen_height_map(&mut self, iterations: Option<u32>) {
        let (width, height) = self.get_size();

        // Checks if nothing was passed in or the iterations is an invalid value
        let iterations = iterations.unwrap_or(1);
        let iterations = if iterations < 1 { 1 } else { iterations };

        // More iterations = Less likely of a single tile, however it hinders CPU performance
        for _ in 0..iterations {
            for x in 0..width {
                for y in 0..height {
                    let current_texture = self.get_value(x, y).unwrap();
                    if current_texture == NON_FILLED_HEIGHT {
                        continue;
                    }

                    let mut neighbour_count = 0;
                    for neighbour_to_check in NEIGHBOURS_TO_CHECK.iter() {
                        // Casted like this to prevent weird overflow arithmetic
                        let nx = x as isize + neighbour_to_check.0 as isize;
                        let ny = y as isize + neighbour_to_check.1 as isize;

                        if let Some(n_texture) = self.get_value(nx as usize, ny as usize) {
                            if n_texture == FILLED_HEIGHT {
                                neighbour_count += 1;
                            }
                        }
                    }

                    // If the tile only has 1 or less neighbours then we need to make it a lower height
                    if neighbour_count < REQUIRED_NEIGHBOURS {
                        self.set_value(x, y, NON_FILLED_HEIGHT);
                    }
                }
            }
        }
    }
}

pub fn generate_texture_atlas(biome_data: &Vec<BiomeData>) {
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

    let packed_map = create_texture_atlas(IMAGE_WIDTH, image_height, tile_size_u32, &biome_data);
    packed_map
        .save(TEXTURE_ATLAS_PATH)
        .expect("Failed to save packed tilemap");
}

pub fn is_water_tile(height: f32, precipiation: f32) -> bool {
    height < WATER_HEIGHT_THRESHOLD && precipiation > WATER_PRECIPITATION_THRESHOLD
}

// Create packed map image buf
fn create_texture_atlas(
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

        let tiles_in_tilemap = (image.width() / tile_size) * (image.height() / tile_size);
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
