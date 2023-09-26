use bevy::prelude::*;

use super::constants::CAMERA_SCALE;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: CAMERA_SCALE,
            ..Default::default()
        },
        ..Default::default()
    });
}
