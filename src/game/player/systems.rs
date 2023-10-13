use bevy::prelude::*;

use crate::game::{camera::components::CameraTarget, world::generation::components::ChunkTarget};

use super::components::MovementController;

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("player/skins/default/default.png"),
            transform: Transform::from_xyz(0.0, 0.0, 2.0),
            ..Default::default()
        })
        .insert((
            MovementController::default(),
            ChunkTarget,
            CameraTarget { priority: 1 },
        ));
}

pub fn manage_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &MovementController), With<MovementController>>,
    time: Res<Time>,
) {
    if let Ok((mut transform, movement_controller)) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        let mut x_delta: f32 = 0.0;
        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            x_delta = -1.0;
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            x_delta = 1.0;
        }

        let mut y_delta: f32 = 0.0;
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            y_delta = 1.0;
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            y_delta = -1.0;
        }

        direction += Vec3::new(x_delta, y_delta, 0.0);
        if !(direction.length() > 0.0) {
            return;
        }

        direction = direction.normalize();
        transform.translation += direction * movement_controller.speed * time.delta_seconds();
    }
}
