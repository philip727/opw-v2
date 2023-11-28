use std::fs;

use bevy::prelude::*;

use crate::states::AppState;

use super::{constants::DEFAULT_DATA_DIR, events::EnterWorld, states::WorldState};

pub fn create_data_directory() {
    if !fs::metadata(DEFAULT_DATA_DIR).is_ok() || !fs::metadata(DEFAULT_DATA_DIR).unwrap().is_dir()
    {
        fs::create_dir(DEFAULT_DATA_DIR).expect(&format!(
            "Failed to create \"{}\"  directory.",
            DEFAULT_DATA_DIR
        ))
    }
}

pub fn enter_world(
    mut commands: Commands,
    mut enter_world_event_reader: EventReader<EnterWorld>,
) {
    for _ in enter_world_event_reader.read() {
        commands.insert_resource(NextState(Some(WorldState::LoadBiomes)));
        commands.insert_resource(NextState(Some(AppState::InGame)));
    }
}
