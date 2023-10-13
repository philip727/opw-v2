use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct TileProperties {
    pub collidable: bool
}

impl Default for TileProperties {
    fn default() -> Self {
        TileProperties { collidable: false }
    }
}
