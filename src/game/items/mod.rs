pub mod components;
pub mod errors;
pub mod helpers;
pub mod resources;
pub mod constants;
pub mod systems;

use bevy::prelude::*;

use self::{resources::ItemDatabase, systems::load_items, helpers::ItemRecord};

use super::world::states::WorldState;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ItemDatabase>()
            .register_type::<ItemRecord>()
            .register_type::<ItemDatabase>()
            .add_systems(OnEnter(WorldState::LoadItems), load_items);
    }
}
