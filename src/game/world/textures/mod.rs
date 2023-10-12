use bevy::prelude::*;

use self::systems::{handle_chunk_rerender, pack_textures};

use super::{generation::events::RequestChunkRender, states::WorldState};

pub mod constants;
pub mod helpers;
pub mod systems;
pub mod resources;

pub struct WorldTexturePlugin;

impl Plugin for WorldTexturePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RequestChunkRender>()
            .add_systems(OnEnter(WorldState::GenerateTextureMap), pack_textures)
            .add_systems(Update, handle_chunk_rerender.run_if(in_state(WorldState::Created)));
    }
}
