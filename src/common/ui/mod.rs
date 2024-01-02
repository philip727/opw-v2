pub mod assets;
pub mod bundles;
pub mod components;
pub mod constants;
pub mod systems;
pub mod macros;

use bevy::prelude::*;

use self::systems::{handle_scrolling_rect_focus, handle_scrolling_rect_scroll};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_scrolling_rect_scroll, handle_scrolling_rect_focus),
        );
    }
}
