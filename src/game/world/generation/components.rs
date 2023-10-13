use bevy::{prelude::*, tasks::Task};

use crate::game::world::{textures::helpers::TextureMap, helpers::ThresholdPos, ruletile::helpers::RuletileMap};

#[derive(Component)]
pub struct ComputeTextureMap(pub Task<(TextureMap, RuletileMap, ThresholdPos)>);

#[derive(Component)]
pub struct Chunk;

#[derive(Component)]
pub struct ChunkTarget;
