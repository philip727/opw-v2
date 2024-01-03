use bevy::prelude::*;

use crate::game::{
    items::{components::Item, resources::ItemDatabase},
    player::components::Player,
};

use super::components::{Inventory, InventorySlot};

pub fn initialize_inventories(
    mut commands: Commands,
    mut inventory_query: Query<(Entity, &mut Inventory), Added<Inventory>>,
) {
    for (entity, mut inventory) in inventory_query.iter_mut() {
        if inventory.initialized() {
            continue;
        }

        inventory.initialize(entity, &mut commands);
    }
}

pub fn test_inputs(
    mut commands: Commands,
    mut inventory_query: Query<&mut Inventory, With<Player>>,
    mut slot_query: Query<&mut InventorySlot>,
    item_database: Res<ItemDatabase>,
    item_query: Query<&Item>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let Ok(mut inventory) = inventory_query.get_single_mut() else {
        return;
    };

    if keyboard_input.just_pressed(KeyCode::J) {
        let Some(item_data) = item_database.get_item_data_by_id("item:common:material:wooden_log")
        else {
            return;
        };

        inventory.add_item(&mut commands, item_data, &mut slot_query, &item_query, 1);
    }

    if keyboard_input.just_pressed(KeyCode::K) {
        let Some(item_data) = item_database.get_item_data_by_id("item:common:material:wooden_log")
        else {
            return;
        };

        inventory.remove_item(&mut commands, item_data, &mut slot_query, &item_query, 1);
    }
}
