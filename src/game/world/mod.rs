use bevy::prelude::*;

use self::{
    biomes::WorldBiomePlugin,
    collisions::WorldCollisionPlugin,
    generation::WorldGenerationPlugin,
    resources::WorldManager,
    states::WorldState,
    systems::{create_data_folder, enter_game},
    textures::WorldTexturePlugin,
};

pub mod biomes;
pub mod collisions;
pub mod constants;
pub mod generation;
pub mod helpers;
pub mod resources;
pub mod ruletile;
pub mod states;
pub mod systems;
pub mod textures;

pub struct WorldPlugins;

impl Plugin for WorldPlugins {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldManager>()
            .add_state::<WorldState>()
            .add_systems(Startup, (create_data_folder, enter_game))
            .add_plugins((
                WorldGenerationPlugin,
                WorldTexturePlugin,
                WorldBiomePlugin,
                WorldCollisionPlugin,
            ));
    }
}
