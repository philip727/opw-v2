pub mod common;
pub mod game;
pub mod math;
pub mod menu;
pub mod states;

use bevy::{prelude::*, window::*};
use bevy_asset_loader::prelude::*;
use bevy_framepace::{FramepaceSettings, Limiter};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use common::ui::{
    assets::{ButtonAssets, PanelAssets},
    UIPlugin,
};
use game::GamePlugins;
use menu::{MenuUIPlugin, assets::MenuAssets};
use states::AppState;

fn main() {
    App::new()
        .add_state::<AppState>()
        .add_loading_state(
            LoadingState::new(AppState::AssetLoading).continue_to_state(AppState::InMenu),
        )
        .add_collection_to_loading_state::<_, MenuAssets>(AppState::AssetLoading)
        .add_collection_to_loading_state::<_, PanelAssets>(AppState::AssetLoading)
        .add_collection_to_loading_state::<_, ButtonAssets>(AppState::AssetLoading)
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
            UIPlugin,
        ))
        .insert_resource(Msaa::Off)
        .run();
}

fn set_frames(mut settings: ResMut<FramepaceSettings>) {
    settings.limiter = Limiter::from_framerate(121.0);
}
