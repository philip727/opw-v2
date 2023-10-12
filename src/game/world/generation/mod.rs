use bevy::prelude::*;

use self::{
    events::RequestTextureMap,
    systems::{generate_texture_maps, handle_texture_map_generation_task, spawn_chunk},
};

use super::states::WorldState;

pub mod components;
pub mod constants;
pub mod events;
pub mod helpers;
pub mod systems;

pub struct WorldGenerationPlugin;

impl Plugin for WorldGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RequestTextureMap>()
            .add_systems(OnEnter(WorldState::Created), spawn_chunk)
            .add_systems(
                Update,
                (generate_texture_maps, handle_texture_map_generation_task)
                    .run_if(in_state(WorldState::Created)),
            );
    }
}
