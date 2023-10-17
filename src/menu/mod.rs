use bevy::prelude::*;

use crate::states::AppState;

use self::systems::{cleanup_menu_ui, handle_play_button, spawn_menu_ui};

pub mod components;
pub mod systems;

pub struct MenuUIPlugin;

impl Plugin for MenuUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_menu_ui)
            .add_systems(OnExit(AppState::InMenu), cleanup_menu_ui)
            .add_systems(
                Update,
                handle_play_button.run_if(in_state(AppState::InMenu)),
            );
    }
}
