use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_ecs_tilemap::TilemapPlugin;

use self::{
    camera::CameraPlugin,
    common::{animation::AnimationPlugin, velocity::VelocityPlugin},
    player::PlayerPlugins,
    world::WorldPlugins, inventory::InventoryPlugin, items::ItemPlugin,
};

pub mod camera;
pub mod common;
pub mod player;
pub mod world;
pub mod items;
pub mod inventory;

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();

        group = group
            .add(TilemapPlugin)
            .add(CameraPlugin)
            .add(InventoryPlugin)
            .add(ItemPlugin)
            .add(PlayerPlugins)
            .add(WorldPlugins)
            .add(AnimationPlugin)
            .add(VelocityPlugin);

        group
    }
}
