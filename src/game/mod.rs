use bevy::{app::PluginGroupBuilder, prelude::PluginGroup};
use bevy_ecs_tilemap::TilemapPlugin;

use self::{
    camera::CameraPlugin,
    common::{animation::AnimationPlugin, velocity::VelocityPlugin},
    player::PlayerPlugins,
    world::WorldPlugins,
};

pub mod camera;
pub mod common;
pub mod player;
pub mod world;

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();

        group = group
            .add(TilemapPlugin)
            .add(CameraPlugin)
            .add(WorldPlugins)
            .add(PlayerPlugins)
            .add(AnimationPlugin)
            .add(VelocityPlugin);

        group
    }
}
