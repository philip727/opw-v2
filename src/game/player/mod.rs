use bevy::prelude::*;

use self::systems::{manage_movement, spawn_player};

use super::world::states::WorldState;

pub mod components;
pub mod systems;

pub struct PlayerPlugins;

impl Plugin for PlayerPlugins {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(WorldState::Created), spawn_player)
            .add_systems(
                Update,
                manage_movement.run_if(in_state(WorldState::Created)),
            );
    }
}
