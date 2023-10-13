use bevy::{prelude::*, tasks::Task};

use crate::game::world::{textures::helpers::TextureMap, helpers::{WorldPos, ThresholdPos}};

#[derive(Component)]
pub struct ComputeTextureMap(pub Task<(TextureMap, ThresholdPos)>);

#[derive(Component)]
pub struct Chunk;

#[derive(Component)]
pub struct ChunkTarget;
