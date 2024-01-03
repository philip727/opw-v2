use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_ecs_tilemap::TilemapPlugin;

use self::{
    camera::CameraPlugin,
    common::{animation::AnimationPlugin, velocity::VelocityPlugin},
    inventory::{ui::InventoryUIPlugin, InventoryPlugin},
    items::ItemPlugin,
    player::PlayerPlugins,
    world::WorldPlugins,
};

pub mod camera;
pub mod common;
pub mod inventory;
pub mod items;
pub mod player;
pub mod world;
pub mod backpack;

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();

        group = group
            .add(TilemapPlugin)
            .add(CameraPlugin)
            .add(InventoryPlugin)
            .add(InventoryUIPlugin)
            .add(ItemPlugin)
            .add(PlayerPlugins)
            .add(WorldPlugins)
            .add(AnimationPlugin)
            .add(VelocityPlugin);

        group
    }
}
