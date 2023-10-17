use bevy::prelude::*;

use self::{
    components::MovementController,
    systems::{manage_direction, manage_movement, manage_state_machine, spawn_player},
};

use super::world::states::WorldState;

pub mod components;
pub mod constants;
pub mod systems;

pub struct PlayerPlugins;

impl Plugin for PlayerPlugins {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(WorldState::Created), spawn_player)
            .add_systems(
                Update,
                (manage_movement, manage_direction, manage_state_machine)
                    .run_if(in_state(WorldState::Created)),
            )
            .register_type::<MovementController>();
    }
}
