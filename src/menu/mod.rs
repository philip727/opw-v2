use bevy::prelude::*;

use crate::{game::world::states::WorldState, states::AppState};

use self::{
    events::SetMenuRootEvent,
    systems::{cleanup_menu_ui, handle_menu_ui_visibility, handle_play_button, spawn_menu_ui},
    world_selection::{
        events::SetWorldSelectionRootEvent,
        resources::WorldSelectionManager,
        systems::{
            cleanup_world_selection_ui, create_saves_directory, handle_selecting_world,
            handle_world_selection_ui_visibility, populate_worlds_container,
            spawn_world_selection_ui,
        },
    },
};

pub mod assets;
pub mod components;
pub mod constants;
pub mod events;
pub mod helpers;
pub mod systems;
pub mod world_selection;

pub struct MenuUIPlugin;

impl Plugin for MenuUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SetMenuRootEvent>()
            .add_event::<SetWorldSelectionRootEvent>()
            .init_resource::<WorldSelectionManager>()
            .add_systems(Startup, create_saves_directory)
            .add_systems(
                OnEnter(AppState::InMenu),
                (
                    // Main Screen
                    spawn_menu_ui,
                    // World Selection Screen
                    spawn_world_selection_ui,
                ),
            )
            .add_systems(
                OnEnter(WorldState::Created),
                (
                    // Main Screen
                    cleanup_menu_ui,
                    // World Selection Screen
                    cleanup_world_selection_ui,
                ),
            )
            .add_systems(
                Update,
                (
                    // Main Screen
                    handle_play_button,
                    handle_menu_ui_visibility,
                    // World Selection Screen
                    populate_worlds_container,
                    handle_world_selection_ui_visibility,
                    handle_selecting_world,
                )
                    .run_if(in_state(AppState::InMenu)),
            );
    }
}
