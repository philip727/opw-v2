use bevy::{utils::HashMap, prelude::*};

use crate::game::world::helpers::ThresholdPos;

use super::helpers::TextureMap;

#[derive(Resource, Default)]
pub struct WorldTextureManager {
    pub cached_texture_maps: HashMap<ThresholdPos, TextureMap>,
}
