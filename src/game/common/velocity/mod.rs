use bevy::prelude::*;

use self::systems::calculate_displacement;

pub mod components;
pub mod systems;

pub struct VelocityPlugin;

impl Plugin for VelocityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            calculate_displacement,
        );
    }
}
