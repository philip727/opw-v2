use bevy::prelude::*;

use self::systems::{handle_chunk_rerender, pack_textures};

use super::generation::events::RequestChunkRender;

pub mod constants;
pub mod helpers;
pub mod systems;

pub struct WorldTexturePlugin;

impl Plugin for WorldTexturePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RequestChunkRender>()
            .add_systems(Startup, pack_textures)
            .add_systems(Update, handle_chunk_rerender);
    }
}
