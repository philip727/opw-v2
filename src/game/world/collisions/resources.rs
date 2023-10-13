use bevy::{prelude::*, utils::HashMap};

use crate::game::world::{helpers::ThresholdPos, ruletile::helpers::RuletileMap};

#[derive(Resource, Default)]
pub struct WorldCollisionManager {
    pub cached_ruletile_maps: HashMap<ThresholdPos, RuletileMap>,
}
