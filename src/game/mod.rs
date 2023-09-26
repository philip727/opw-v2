use bevy::{app::PluginGroupBuilder, prelude::PluginGroup};

use self::camera::CameraPlugin;

pub mod camera;
pub mod world;

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();

        group = group.add(CameraPlugin);

        group
    }
}
