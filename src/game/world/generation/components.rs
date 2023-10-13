use bevy::{prelude::*, tasks::Task};

use crate::game::world::{textures::helpers::TextureMap, helpers::WorldPos};

#[derive(Component)]
pub struct ComputeTextureMap(pub Task<(TextureMap, Vec3)>);

#[derive(Component)]
pub struct Chunk;

#[derive(Component)]
pub struct ChunkTarget;
