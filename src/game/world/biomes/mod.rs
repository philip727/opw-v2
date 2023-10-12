use bevy::prelude::*;

use self::{resources::BiomeManager, systems::setup_biome_data};

use super::states::WorldState;

pub mod constants;
pub mod helpers;
pub mod resources;
pub mod systems;

pub struct WorldBiomePlugin;

impl Plugin for WorldBiomePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BiomeManager>()
            .add_systems(OnEnter(WorldState::LoadBiomes), setup_biome_data);
    }
}
