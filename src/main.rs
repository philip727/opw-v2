pub mod game;
pub mod math;
pub mod menu;

use bevy::{prelude::*, window::*};
use bevy_framepace::{FramepaceSettings, Limiter};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use game::GamePlugins;
use menu::MenuUIPlugin;

fn main() {
    App::new()
        // Dev Plugins
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        mode: WindowMode::Windowed,
                        title: "ow-sb-v2".into(),
                        resizable: true,
                        present_mode: PresentMode::Immediate,
                        window_level: WindowLevel::Normal,
                        resolution: WindowResolution::new(1920.0, 1080.0),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
            WorldInspectorPlugin::new(),
            MenuUIPlugin,
            GamePlugins,
        ))
        .insert_resource(Msaa::Off)
        .run();
}

fn set_frames(mut settings: ResMut<FramepaceSettings>) {
    settings.limiter = Limiter::from_framerate(121.0);
}
