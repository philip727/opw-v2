use bevy::prelude::*;

use super::{
    components::CameraTarget,
    constants::{CAMERA_POS_Z, CAMERA_SCALE},
};

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: CAMERA_SCALE,
            ..Default::default()
        },
        transform: Transform {
            translation: Transform::from_xyz(0.0f32, 0.0f32, CAMERA_POS_Z).translation,
            ..Default::default()
        },
        ..Default::default()
    });
}

pub fn update_camera_from_target(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    target_query: Query<&Transform, (With<CameraTarget>, Without<Camera>)>,
) {
    let mut camera_transform = camera_query.single_mut();

    for target_transform in target_query.iter() {
        camera_transform.translation = Vec3::new(
            target_transform.translation.x,
            target_transform.translation.y,
            CAMERA_POS_Z,
        );
    }
}
