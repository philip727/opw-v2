pub mod components;
pub mod constants;
pub mod systems;

use bevy::prelude::*;

use self::systems::{spawn_camera, update_camera_from_target};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, update_camera_from_target);
    }
}
