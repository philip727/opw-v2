use bevy::prelude::*;

use crate::game::world::{generation::constants::TILE_SIZE, states::WorldState};
use image::io::Reader as ImageReader;

use super::{helpers::load_biome_data, resources::BiomeManager};

pub fn setup_biome_data(mut biome_manager: ResMut<BiomeManager>, mut commands: Commands) {
    info!("Biome data loaded");
    let tile_size_u32 = TILE_SIZE as u32;
    let biome_data = load_biome_data();

    // Creates the offset for each tilemap by id
    let mut tile_offset = 0u32;
    for biome in biome_data.iter() {
        let image = ImageReader::open(&biome.texture_location)
            .expect(&format!("Failed to load \"{}\"", biome.texture_location))
            .decode()
            .expect(&format!("Failed to load \"{}\"", biome.texture_location));

        let tiles_in_image = (image.width() / tile_size_u32) * (image.height() / tile_size_u32);
        biome_manager.offsets.insert(biome.id.clone(), tile_offset);
        tile_offset += tiles_in_image;
    }

    biome_manager.loaded = biome_data;
    commands.insert_resource(NextState(Some(WorldState::GenerateTextureMap)));
}
