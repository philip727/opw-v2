use bevy::prelude::*;

use self::{
    animations::systems::handle_tile_animations,
    resources::WorldTextureManager,
    systems::{handle_chunk_rerender, pack_textures},
};

use super::{generation::events::RequestChunkRender, states::WorldState};

pub mod animations;
pub mod constants;
pub mod helpers;
pub mod resources;
pub mod systems;

pub struct WorldTexturePlugin;

impl Plugin for WorldTexturePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RequestChunkRender>()
            .init_resource::<WorldTextureManager>()
            .add_systems(OnEnter(WorldState::GenerateTextureMap), pack_textures)
            .add_systems(
                Update,
                (handle_chunk_rerender, handle_tile_animations)
                    .run_if(in_state(WorldState::Created)),
            );
    }
}
