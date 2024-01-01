use bevy::prelude::*;

use crate::game::{
    inventory::components::{Inventory, InventorySlot},
    player::components::Player,
};

use super::events::CreateInventoryUI;

pub fn create_inventory_uis(
    mut create_inventory_reader: EventReader<CreateInventoryUI>,
    inventory_query: Query<&Children, With<Inventory>>,
    slot_query: Query<&InventorySlot>,
) {
    for event in create_inventory_reader.read() {
        let Ok(slots) = inventory_query.get(event.inventory_entity) else {
            continue;
        };

        for slot in slots.iter() {
            let Ok(slot) = slot_query.get(*slot) else {
                continue;
            };

            info!("{:?}", slot);
        }
    }
}

pub fn open_inventory(
    keyboard_input: Res<Input<KeyCode>>,
    mut create_inventory_ui_writer: EventWriter<CreateInventoryUI>,
    inventory_query: Query<(Entity, &Inventory), With<Player>>,
) {
    if keyboard_input.just_pressed(KeyCode::Tab) {
        let entity = inventory_query.single().0;

        create_inventory_ui_writer.send(CreateInventoryUI {
            inventory_entity: entity,
        })
    }
}
