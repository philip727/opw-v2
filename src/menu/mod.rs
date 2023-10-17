use bevy::prelude::*;

use self::systems::spawn_menu_ui;

pub mod systems;

pub struct MenuUIPlugin;

impl Plugin for MenuUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_menu_ui);
    }
}
