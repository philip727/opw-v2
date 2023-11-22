pub mod assets;
pub mod components;
pub mod constants;
pub mod systems;

use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
 //       app.add_systems(OnEnter(AppState::InMenu), setup_ui_scaling);
    }
}
