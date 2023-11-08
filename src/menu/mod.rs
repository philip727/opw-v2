use bevy::prelude::*;

use crate::{states::AppState, game::world::states::WorldState};

use self::{systems::{cleanup_menu_ui, handle_play_button, spawn_menu_ui, handle_menu_ui_visibility}, events::SetMenuRootEvent};

pub mod components;
pub mod systems;
pub mod events;
pub mod constants;
pub mod helpers;

pub struct MenuUIPlugin;

impl Plugin for MenuUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SetMenuRootEvent>().add_systems(OnEnter(AppState::InMenu), spawn_menu_ui)
            .add_systems(OnEnter(WorldState::Created), cleanup_menu_ui)
            .add_systems(
                Update,
                (handle_play_button, handle_menu_ui_visibility).run_if(in_state(AppState::InMenu)),
            );
    }
}
