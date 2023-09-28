use bevy::prelude::*;
use noise::{Fbm, MultiFractal, Perlin, utils::{PlaneMapBuilder, NoiseMapBuilder, NoiseMap}};

use crate::game::world::generation::constants::CHUNK_SIZE;

pub fn generate_perlin_noise(
    x: i32,
    y: i32,
    seed: u32,
    offset: Vec2,
    octaves: usize,
    lacunarity: f64,
    persistence: f64,
    frequency: f64,
) -> NoiseMap {
    let fbm = Fbm::<Perlin>::new(seed)
        .set_octaves(octaves)
        .set_lacunarity(lacunarity)
        .set_persistence(persistence)
        .set_frequency(frequency);

    let lower_x_bound = x as f64 + offset.x as f64;
    let lower_y_bound = y as f64 + offset.y as f64;

    let upper_x_bound = (x + CHUNK_SIZE as i32) as f64 + offset.x as f64;
    let upper_y_bound = (y + CHUNK_SIZE as i32) as f64 + offset.y as f64;

    // Creates a 2d noise map
    let noise_map = PlaneMapBuilder::<_, 2>::new(&fbm)
        .set_size(CHUNK_SIZE as usize, CHUNK_SIZE as usize)
        .set_x_bounds(lower_x_bound, upper_x_bound)
        .set_y_bounds(lower_y_bound, upper_y_bound)
        .build();

    noise_map
}
