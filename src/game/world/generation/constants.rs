use bevy::prelude::Vec2;

pub const TILE_SIZE: f32 = 16.0;

pub const CHUNK_SIZE: u32 = 128;
pub const CHUNK_Z_POS: f32 = 0.0;
pub const CHUNK_RERENDER_DISTANCE_THRESHOLD: f32 = 32.0;

pub const HEIGHT_OCTAVES: usize = 4;
pub const HEIGHT_LACUNARITY: f64 = 10.0;
pub const HEIGHT_PERSISTENCE: f64 = 0.1;
pub const HEIGHT_FREQUENCY: f64 = 0.02;
pub const HEIGHT_OFFSET: Vec2 = Vec2 { x: 0.0, y: 0.0 };
pub const HEIGHT_SCALE: f64 = 1.2;

pub const TEMPERATURE_OCTAVES: usize = 6;
pub const TEMPERATURE_LACUNARITY: f64 = 9.5;
pub const TEMPERATURE_PERSISTENCE: f64 = 0.12;
pub const TEMPERATURE_FREQUENCY: f64 = 0.017;
pub const TEMPERATURE_OFFSET: Vec2 = Vec2 { x: 512.0, y: -320.0 };
pub const TEMPERATURE_SCALE: f64 = 5.0;

pub const PRECIPITATION_OCTAVES: usize = 6;
pub const PRECIPITATION_LACUNARITY: f64 = 10.5;
pub const PRECIPITATION_PERSISTENCE: f64 = 0.09;
pub const PRECIPITATION_FREQUENCY: f64 = 0.022;
pub const PRECIPITATION_OFFSET: Vec2 = Vec2 { x: -54.0, y: 212.0 };
pub const PRECIPITATION_SCALE: f64 = 5.8;

pub const WATER_HEIGHT_THRESHOLD: f32 = 0.35;
pub const WATER_PRECIPITATION_THRESHOLD: f32 = 0.27;
