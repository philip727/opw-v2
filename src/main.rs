pub mod game;

use bevy::{
    prelude::*,
    window::{PresentMode, WindowLevel, WindowMode},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        // Dev Plugins
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        mode: WindowMode::Fullscreen,
                        title: "ow-sb-v2".into(),
                        resizable: true,
                        present_mode: PresentMode::AutoVsync,
                        window_level: WindowLevel::Normal,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
            WorldInspectorPlugin::new(),
        ))
        .run();
}
