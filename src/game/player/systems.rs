use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;

use crate::game::{
    camera::components::CameraTarget,
    world::{
        collisions::{components::TileProperties, helpers::colliding_with_wall},
        generation::components::{Chunk, ChunkTarget},
    },
};

use super::{components::MovementController, constants::PLAYER_POS_Z};

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("player/skins/default/default.png"),
            transform: Transform::from_xyz(0.0, 0.0, PLAYER_POS_Z),
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
    mut player_query: Query<(&mut Transform, &MovementController)>,
    tile_query: Query<(&TileProperties, &TilePos)>,
    chunk_query: Query<&Transform, (With<Chunk>, Without<MovementController>)>,
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

        let chunk_transform = chunk_query.single();
        let target = transform.translation + Vec3::new(x_delta, 0.0, 0.0);
        if !colliding_with_wall(target, &tile_query, &chunk_transform) {
            direction += Vec3::new(x_delta, 0.0, 0.0);
        }

        let target = transform.translation + Vec3::new(0.0, y_delta, 0.0);
        if !colliding_with_wall(target, &tile_query, &chunk_transform) {
            direction += Vec3::new(0.0, y_delta, 0.0);
        }

        if !(direction.length() > 0.0) {
            return;
        }

        direction = direction.normalize();
        transform.translation += direction * movement_controller.speed * time.delta_seconds();
    }
}
