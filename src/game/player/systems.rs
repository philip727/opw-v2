use bevy::{prelude::*, window::PrimaryWindow};
use bevy_ecs_tilemap::tiles::TilePos;

use crate::game::{
    camera::components::CameraTarget,
    world::{
        collisions::{components::TileProperties, helpers::colliding_with_wall},
        generation::components::{Chunk, ChunkTarget},
    },
};

use super::{
    components::{DirectionController, MovementController},
    constants::PLAYER_POS_Z,
};

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Name::new("Player"),
        SpriteBundle {
            texture: asset_server.load("player/skins/default/default.png"),
            transform: Transform::from_xyz(0.0, 0.0, PLAYER_POS_Z),
            ..Default::default()
        },
        MovementController::default(),
        DirectionController,
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

        // Allows us to slide against walls
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

        transform.translation +=
            direction.normalize() * movement_controller.speed * time.delta_seconds();
    }
}

pub fn manage_direction(
    mut player_query: Query<(&Transform, &mut Sprite), With<DirectionController>>,
    camera_query: Query<(&GlobalTransform, &Camera), Without<DirectionController>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let (player_transform, mut sprite) = player_query.single_mut();
    let (camera_transform, camera) = camera_query.single();
    let window = window_query.single();

    if let (Some(screen_position), Some(cursor_position)) = (
        camera.world_to_viewport(camera_transform, player_transform.translation),
        window.cursor_position(),
    ) {
        sprite.flip_x = cursor_position.x < screen_position.x;
    }
}
