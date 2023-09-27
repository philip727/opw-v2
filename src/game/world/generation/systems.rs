use bevy::prelude::*;

use super::helpers::{IntoChunkPos, create_chunk_tilemap};

pub fn spawn_chunk(mut commands: Commands, asset_server: Res<AssetServer>) {
    let start_pos = Vec2 { x: 0.0, y: 0.0 };
    let chunk_pos = start_pos.to_chunk_pos();

    let chunk_entity = create_chunk_tilemap(&mut commands, &asset_server, &chunk_pos);
}
