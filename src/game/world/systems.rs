use std::fs;

use bevy::prelude::*;

use crate::states::AppState;

use super::{constants::DEFAULT_DATA_DIR, events::EnterWorldEvent, states::WorldState};

pub fn create_data_folder() {
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
    mut enter_world_event_reader: EventReader<EnterWorldEvent>,
) {
    for _ in enter_world_event_reader.iter() {
        commands.insert_resource(NextState(Some(WorldState::LoadBiomes)));
        commands.insert_resource(NextState(Some(AppState::InGame)));
    }
}
