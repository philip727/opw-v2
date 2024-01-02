use bevy::prelude::*;

use crate::{
    create_ui,
    game::{
        inventory::components::{Inventory, InventorySlot},
        player::components::Player,
    },
};

use super::{events::CreateInventoryUI, resources::InventoryUIManager};

pub fn create_inventories_root_ui(
    mut commands: Commands,
    mut inventory_ui_manager: ResMut<InventoryUIManager>,
) {
    create_ui!(GridCenter->root);
    root.style = Style {
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        grid_template_columns: vec![GridTrack::auto()],
        grid_template_rows: vec![GridTrack::auto()],
        ..root.style
    };

    inventory_ui_manager.inventory_parent = Some(commands.spawn(root).id());
}

pub fn cleanup_inventories_root_ui(
    mut commands: Commands,
    inventory_ui_manager: Res<InventoryUIManager>,
) {
    commands
        .entity(inventory_ui_manager.root())
        .despawn_recursive();
}

pub fn toggle_inventory_uis(
    mut commands: Commands,
    mut create_inventory_reader: EventReader<CreateInventoryUI>,
    inventory_query: Query<&Children, With<Inventory>>,
    slot_query: Query<&InventorySlot>,
    mut inventory_ui_manager: ResMut<InventoryUIManager>,
) {
    for event in create_inventory_reader.read() {
        let inventory_entity = event.inventory_entity;
        let Ok(slots) = inventory_query.get(inventory_entity) else {
            continue;
        };

        // Despawns the inventory ui if it is open already
        if inventory_ui_manager.is_inventory_open(&inventory_entity) {
            let inventory_ui_entity = inventory_ui_manager
                .inventory_to_ui(&inventory_entity)
                .unwrap();

            commands.entity(*inventory_ui_entity).despawn_recursive();
            inventory_ui_manager.destroy_inventory(&inventory_entity);
            continue;
        }

        // Creates the background
        create_ui!(Node->inventory_node);
        inventory_node.style = Style {
            width: Val::Percent(92.),
            height: Val::Percent(92.),
            ..inventory_node.style
        };
        inventory_node.background_color = BackgroundColor(Color::WHITE);

        // Adds the inventory ui to the inventory root display and creates references
        let inventory_ui_entity = commands.spawn(inventory_node).id();
        commands
            .entity(inventory_ui_manager.root())
            .push_children(&[inventory_ui_entity]);
        inventory_ui_manager.add_inventory(inventory_entity, inventory_ui_entity);

        create_ui!(Node->slot_holder_node);
        slot_holder_node.style = Style {
            flex_wrap: FlexWrap::Wrap,
            align_items: AlignItems::Start,
            justify_items: JustifyItems::Start,
            width: Val::Percent(100.),
            ..slot_holder_node.style
        };

        let slot_holder_entity = commands.spawn(slot_holder_node).id();
        commands
            .entity(inventory_ui_entity)
            .push_children(&[slot_holder_entity]);

        // Creates the inventory slots that belong to the parent ui entity
        for slot in slots.iter() {
            let Ok(slot) = slot_query.get(*slot) else {
                continue;
            };

            // Creates slot node
            create_ui!(Btn->slot_node);
            slot_node.style = Style {
                width: Val::Px(128.),
                height: Val::Px(128.),
                ..slot_node.style
            };
            slot_node.background_color = BackgroundColor(Color::BLACK);

            let slot_ui_entity = commands.spawn(slot_node).id();
            commands
                .entity(slot_holder_entity)
                .push_children(&[slot_ui_entity]);

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
