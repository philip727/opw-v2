use bevy::prelude::*;

use crate::{game::world::states::WorldState, states::AppState};

use self::{
    events::UpdatePage,
    resources::MenuManager,
    systems::*,
    world_selection::{
        events::{StartSelectedWorld, UpdateSelectedWorld},
        resources::WorldSelectionManager,
        systems::*,
    },
};

pub mod assets;
pub mod components;
pub mod constants;
pub mod events;
pub mod helpers;
pub mod resources;
pub mod systems;
pub mod world_selection;

pub struct MenuUIPlugin;

impl Plugin for MenuUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UpdatePage>()
            .add_event::<StartSelectedWorld>()
            .add_event::<UpdateSelectedWorld>()
            .init_resource::<MenuManager>()
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
                    handle_page_updates,
                    // Main Screen
                    menu_ui_visibility,
                    // World Selection Screen
                    world_selection_ui_visibility,
                    populate_worlds,
                    start_selected_world,
                    update_selected_world,
                )
                    .run_if(in_state(AppState::InMenu)),
            );
    }
}
