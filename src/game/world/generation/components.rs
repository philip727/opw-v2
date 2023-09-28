use bevy::{prelude::*, tasks::Task};
use noise::utils::NoiseMap;

use crate::game::world::{textures::helpers::TextureMap, helpers::WorldPos};

use super::helpers::{HeightNoiseMap, TemperatureNoiseMap, PrecipitationNoiseMap};

#[derive(Component)]
pub struct ComputeTextureMap(pub Task<(TextureMap, WorldPos)>);

#[derive(Component)]
pub struct Chunk;
