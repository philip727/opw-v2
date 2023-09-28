use bevy::prelude::*;

use super::constants::CAMERA_SCALE;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: CAMERA_SCALE,
            ..Default::default()
        },
        transform: Transform {
            translation: Transform::from_xyz(0.0f32, 0.0f32, 999.0f32).translation,
            ..Default::default()
        },
        ..Default::default()
    });
}
