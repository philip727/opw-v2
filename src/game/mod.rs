use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_ecs_tilemap::TilemapPlugin;

use self::{
    camera::CameraPlugin,
    common::{animation::AnimationPlugin, velocity::VelocityPlugin},
    events::EnterWorldEvent,
    player::PlayerPlugins,
    world::WorldPlugins,
};

pub mod camera;
pub mod common;
pub mod events;
pub mod player;
pub mod world;

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();

        group = group
            .add(GameFunctionPlugin)
            .add(TilemapPlugin)
            .add(CameraPlugin)
            .add(PlayerPlugins)
            .add(WorldPlugins)
            .add(AnimationPlugin)
            .add(VelocityPlugin);

        group
    }
}

pub struct GameFunctionPlugin;

impl Plugin for GameFunctionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EnterWorldEvent>();
    }
}
