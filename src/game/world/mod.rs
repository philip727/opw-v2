use bevy::prelude::*;

use crate::states::AppState;

use self::{
    biomes::WorldBiomePlugin,
    collisions::WorldCollisionPlugin,
    events::EnterWorldEvent,
    generation::WorldGenerationPlugin,
    resources::WorldManager,
    states::WorldState,
    systems::{create_data_directory, enter_world},
    textures::WorldTexturePlugin,
};

pub mod biomes;
pub mod collisions;
pub mod constants;
pub mod events;
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
            .add_event::<EnterWorldEvent>()
            .add_state::<WorldState>()
            .add_systems(Startup, create_data_directory)
            .add_systems(Update, enter_world.run_if(in_state(AppState::InMenu)))
            .add_plugins((
                WorldGenerationPlugin,
                WorldTexturePlugin,
                WorldBiomePlugin,
                WorldCollisionPlugin,
            ));
    }
}
