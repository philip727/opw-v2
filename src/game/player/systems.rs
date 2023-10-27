use bevy::{prelude::*, window::PrimaryWindow};
use bevy_ecs_tilemap::tiles::TilePos;

use crate::{
    common::state_machine::components::StateMachine,
    game::{
        camera::components::CameraTarget,
        common::{
            animation::helpers::AnimationState, skin::helpers::EntitySkin,
            velocity::components::Velocity,
        },
        world::{
            collisions::{components::TileProperties, helpers::colliding_with_wall},
            generation::{
                components::{Chunk, ChunkTarget},
                constants::TILE_SIZE,
            },
        },
    },
};

use super::{
    components::{DirectionController, MovementController, Player},
    constants::PLAYER_POS_Z,
};

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let entity_skin = EntitySkin::load_from_path("./data/skins/default".into()).unwrap();
    let texture_atlas = entity_skin.generate_texture_atlas(&asset_server);
    let animation_state: StateMachine<AnimationState> = entity_skin.into();
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn((
        Name::new("Player"),
        Player,
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_xyz(0.0, 0.0, PLAYER_POS_Z),
            ..Default::default()
        },
        animation_state,
        MovementController::default(),
        DirectionController,
        ChunkTarget,
        CameraTarget { priority: 1 },
        Velocity::new(Vec3::new(0.0, 0.0, PLAYER_POS_Z)),
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
        if !colliding_with_wall(
            Vec2::splat(TILE_SIZE),
            target,
            &tile_query,
            &chunk_transform,
        ) {
            direction += Vec3::new(x_delta, 0.0, 0.0);
        }

        let target = transform.translation + Vec3::new(0.0, y_delta, 0.0);
        if !colliding_with_wall(
            Vec2::splat(TILE_SIZE),
            target,
            &tile_query,
            &chunk_transform,
        ) {
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
    mut player_query: Query<(&Transform, &mut TextureAtlasSprite), With<DirectionController>>,
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

pub fn manage_state_machine(
    mut player_query: Query<(&mut StateMachine<AnimationState>, &Velocity), With<Player>>,
) {
    let (mut state_machine, velocity) = player_query.single_mut();

    //info!("{}", velocity.displacement.normalize().length());

    if velocity.displacement.normalize().length() > 0.0 {
        if !state_machine.is_current_state("Run".into()) {
            state_machine
                .set_state("Run".into())
                .expect("Passed in invalid state");
        }

        return;
    }

    if state_machine.is_current_state("Idle".into()) {
        return;
    }

    state_machine
        .set_state("Idle".into())
        .expect("Passed in invalid state");
}
