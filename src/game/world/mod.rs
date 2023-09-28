use bevy::prelude::*;

use self::{
    generation::WorldGenerationPlugin, resources::WorldManager, textures::WorldTexturePlugin,
};

pub mod generation;
pub mod helpers;
pub mod resources;
pub mod textures;

pub struct WorldPlugins;

impl Plugin for WorldPlugins {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldManager>()
            .add_plugins((WorldGenerationPlugin, WorldTexturePlugin));
    }
}
