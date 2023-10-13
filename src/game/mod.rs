use bevy::{app::PluginGroupBuilder, prelude::PluginGroup};
use bevy_ecs_tilemap::TilemapPlugin;

use self::{camera::CameraPlugin, player::PlayerPlugins, world::WorldPlugins};

pub mod camera;
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
            .add(PlayerPlugins);

        group
    }
}
