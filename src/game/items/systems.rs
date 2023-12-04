use bevy::prelude::*;

use crate::game::world::states::WorldState;

use super::{constants::ITEMS_JSON_DIR, resources::ItemDatabase};

pub fn load_items(mut commands: Commands, mut item_database: ResMut<ItemDatabase>) {
    info!("Initializing item database");
    item_database.initialize(ITEMS_JSON_DIR.into());

    info!("Finished loading items");
    commands.insert_resource(NextState(Some(WorldState::LoadPlayer)));
}
