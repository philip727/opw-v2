use bevy::prelude::*;

use self::systems::spawn_chunk;

pub mod constants;
pub mod helpers;
pub mod systems;

pub struct WorldGenerationPlugin;

impl Plugin for WorldGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_chunk);
    }
}
