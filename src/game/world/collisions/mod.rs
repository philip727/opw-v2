use bevy::prelude::*;

use self::{resources::WorldCollisionManager, systems::handle_collision_update};

use super::states::WorldState;

pub mod components;
pub mod resources;
pub mod systems;
pub mod helpers;

pub struct WorldCollisionPlugin;

impl Plugin for WorldCollisionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldCollisionManager>().add_systems(
            Update,
            handle_collision_update.run_if(in_state(WorldState::Created)),
        );
    }
}
