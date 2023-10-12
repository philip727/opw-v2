use bevy::{prelude::*, tasks::Task};

use crate::game::world::{textures::helpers::TextureMap, helpers::WorldPos};

#[derive(Component)]
pub struct ComputeTextureMap(pub Task<(TextureMap, WorldPos)>);

#[derive(Component)]
pub struct Chunk;
