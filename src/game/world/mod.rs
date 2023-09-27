use bevy::prelude::*;

use self::generation::WorldGenerationPlugin;

pub mod generation;

pub struct WorldPlugins;

impl Plugin for WorldPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((WorldGenerationPlugin));
    }
}
