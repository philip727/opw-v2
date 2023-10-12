use std::fs;

use bevy::prelude::*;

use super::{constants::DEFAULT_DATA_DIR, states::WorldState};

pub fn create_data_folder() {
    if !fs::metadata(DEFAULT_DATA_DIR).is_ok() || !fs::metadata(DEFAULT_DATA_DIR).unwrap().is_dir()
    {
        fs::create_dir(DEFAULT_DATA_DIR).expect(&format!(
            "Failed to create \"{}\"  directory.",
            DEFAULT_DATA_DIR
        ))
    }
}

pub fn enter_game(mut commands: Commands) {
    commands.insert_resource(NextState(Some(WorldState::LoadBiomes)));
}
