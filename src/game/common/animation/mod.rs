use bevy::prelude::*;



use self::systems::handle_animation_machines;

pub mod components;
pub mod helpers;
pub mod systems;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_animation_machines,
        );
    }
}
