use bevy::prelude::*;

use self::{
    generation::WorldGenerationPlugin, resources::WorldManager, textures::WorldTexturePlugin, systems::{create_data_folder, enter_game}, states::WorldState, biomes::WorldBiomePlugin, collisions::WorldCollisionPlugin,
};

pub mod generation;
pub mod helpers;
pub mod resources;
pub mod textures;
pub mod biomes;
pub mod systems;
pub mod constants;
pub mod states;
pub mod ruletile;
pub mod collisions;

pub struct WorldPlugins;

impl Plugin for WorldPlugins {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldManager>()
            .add_state::<WorldState>()
            .add_systems(Startup, (create_data_folder, enter_game))
            .add_plugins((WorldGenerationPlugin, WorldTexturePlugin, WorldBiomePlugin, WorldCollisionPlugin));
    }
}
