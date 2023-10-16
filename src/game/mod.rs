use bevy::{app::PluginGroupBuilder, prelude::PluginGroup};
use bevy_ecs_tilemap::TilemapPlugin;

use self::{
    animation::AnimationPlugin, camera::CameraPlugin, player::PlayerPlugins, world::WorldPlugins,
};

pub mod animation;
pub mod camera;
pub mod player;
pub mod world;
pub mod entity_skin;

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();

        group = group
            .add(TilemapPlugin)
            .add(CameraPlugin)
            .add(WorldPlugins)
            .add(PlayerPlugins)
            .add(AnimationPlugin);

        group
    }
}
