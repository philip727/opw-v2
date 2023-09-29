use bevy::prelude::*;

use self::{
    generation::WorldGenerationPlugin, resources::WorldManager, textures::WorldTexturePlugin, systems::create_data_folder,
};

pub mod generation;
pub mod helpers;
pub mod resources;
pub mod textures;
pub mod biomes;
pub mod systems;
pub mod constants;

pub struct WorldPlugins;

impl Plugin for WorldPlugins {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldManager>()
            .add_systems(Startup, create_data_folder)
            .add_plugins((WorldGenerationPlugin, WorldTexturePlugin));
    }
}
