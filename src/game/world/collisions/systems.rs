use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::{
    game::world::{
        generation::events::RequestChunkRender, resources::WorldManager,
        ruletile::helpers::RuleTile,
    },
    math::map::ValueMap2D,
};

use super::components::TileProperties;

pub fn handle_collision_update(
    mut request_chunk_rerender_reader: EventReader<RequestChunkRender>,
    world_manager: Res<WorldManager>,
    mut tile_query: Query<(&mut TileProperties, &TilePos)>,
) {
    for event in request_chunk_rerender_reader.iter() {
        if let None = world_manager.chunk_entity {
            continue;
        }

        let ruletile_map = &event.ruletile_map;
        for (mut tile_properties, tile_pos) in &mut tile_query {
            // Sets the collision property of each one, depending on the rule, this is so we
            // collide with water
            tile_properties.collidable = match ruletile_map
                .get_value(tile_pos.x as usize, tile_pos.y as usize)
                .unwrap()
            {
                RuleTile::Water => true,
                RuleTile::BottomLeft => true,
                RuleTile::BottomMiddle => true,
                RuleTile::BottomRight => true,
                _ => false,
            }
        }

        info!("Finished collision update");
    }
}
