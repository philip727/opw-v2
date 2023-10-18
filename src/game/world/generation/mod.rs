use bevy::prelude::*;

use crate::states::AppState;

use self::{
    events::RequestTextureMap,
    resources::WorldGenerationManager,
    systems::{
        create_world_generation_settings, handle_chunk_texture_map, request_chunk_texture_map,
        spawn_chunk, update_chunk_pos,
    },
};

use super::states::WorldState;

pub mod components;
pub mod constants;
pub mod events;
pub mod helpers;
pub mod resources;
pub mod systems;

pub struct WorldGenerationPlugin;

impl Plugin for WorldGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldGenerationManager>()
            .add_event::<RequestTextureMap>()
            .add_systems(
                Update,
                create_world_generation_settings.run_if(in_state(AppState::InMenu)),
            )
            .add_systems(OnEnter(WorldState::Created), spawn_chunk)
            .add_systems(
                Update,
                (
                    request_chunk_texture_map,
                    handle_chunk_texture_map,
                    update_chunk_pos,
                )
                    .run_if(in_state(WorldState::Created)),
            );
    }
}
