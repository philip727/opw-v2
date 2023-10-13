use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_ecs_tilemap::tiles::TilePos;

use crate::game::{
    player::constants::PLAYER_POS_Z,
    world::generation::constants::TILE_SIZE,
};

use super::components::TileProperties;

pub fn colliding_with_wall(
    target_pos: Vec3,
    tile_query: &Query<(&TileProperties, &TilePos)>,
    chunk_transform: &Transform,
) -> bool {
    for (tile_properties, transform) in tile_query.iter() {
        if !tile_properties.collidable {
            continue;
        }

        let collision = collide(
            target_pos,
            Vec2::splat(TILE_SIZE),
            Vec3::new(
                chunk_transform.translation.x + transform.x as f32 * TILE_SIZE,
                chunk_transform.translation.y + transform.y as f32 * TILE_SIZE,
                PLAYER_POS_Z,
            ),
            Vec2::splat(TILE_SIZE),
        );

        if collision.is_some() {
            return true;
        }
    }

    false
}
