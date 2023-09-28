use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct WorldManager {
    pub chunk_entity: Option<Entity>,
}
