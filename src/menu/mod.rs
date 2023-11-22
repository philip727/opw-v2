use bevy::prelude::*;

use crate::{game::world::states::WorldState, states::AppState};

use self::{
    events::SetMenuRootEvent,
    systems::{cleanup_menu_ui, handle_menu_ui_visibility, handle_play_button, spawn_menu_ui},
    world_selection::{
        events::SetWorldSelectionRootEvent,
        systems::{
            cleanup_world_selection_ui, handle_world_selection_ui_visibility, spawn_world_selection_ui,
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
            .add_systems(OnEnter(AppState::InMenu), (spawn_menu_ui, spawn_world_selection_ui))
            .add_systems(
                OnEnter(WorldState::Created),
                (cleanup_menu_ui, cleanup_world_selection_ui),
            )
            .add_systems(
                Update,
                (
                    handle_play_button,
                    handle_menu_ui_visibility,
                    handle_world_selection_ui_visibility,
                )
                    .run_if(in_state(AppState::InMenu)),
            );
    }
}
