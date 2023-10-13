use bevy::prelude::*;

use self::{
    events::RequestTextureMap,
    resources::WorldGenerationManager,
    systems::{
        generate_texture_for_chunk, handle_texture_map_generation_task, spawn_chunk,
        update_chunk_from_target,
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
            .add_systems(OnEnter(WorldState::Created), spawn_chunk)
            .add_systems(
                Update,
                (
                    generate_texture_for_chunk,
                    handle_texture_map_generation_task,
                    update_chunk_from_target,
                )
                    .run_if(in_state(WorldState::Created)),
            );
    }
}
