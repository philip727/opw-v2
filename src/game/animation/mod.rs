use bevy::prelude::*;

use self::systems::handle_animation_machines;

use super::world::states::WorldState;

pub mod components;
pub mod helpers;
pub mod systems;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_animation_machines.run_if(in_state(WorldState::Created)),
        );
    }
}
