use bevy::prelude::*;

use crate::{
    create_ui,
    game::{
        inventory::{
            components::{Inventory, InventorySlot},
            ui::components::UIInventorySlot,
        },
        items::{components::Item, resources::ItemDatabase},
        player::components::Player,
    },
};

use super::{events::CreateInventoryUI, resources::InventoryUIManager, components};

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

// Takes in an inventory entity, and either creates or destorys the ui corresponding to it
pub fn toggle_inventory_ui(
    mut commands: Commands,
    mut create_inventory_reader: EventReader<CreateInventoryUI>,
    inventory_query: Query<&Children, With<Inventory>>,
    mut slot_query: Query<&mut InventorySlot>,
    item_query: Query<&Item>,
    mut inventory_ui_manager: ResMut<InventoryUIManager>,
    asset_server: Res<AssetServer>,
    item_database: Res<ItemDatabase>,
) {
    for event in create_inventory_reader.read() {
        let inventory_entity = event.inventory_entity;
        let Ok(slots) = inventory_query.get(inventory_entity) else {
            continue;
        };

        // Despawns the inventory ui corresponding to the inventory passed in
        if inventory_ui_manager.inventory_has_ui(&inventory_entity) {
            // Gets the inventory ui entity according to the inventory
            let inventory_ui_entity = inventory_ui_manager
                .inventory_to_ui(&inventory_entity)
                .unwrap();

            commands.entity(*inventory_ui_entity).despawn_recursive();
            inventory_ui_manager.destroy_inventory_ui(&inventory_entity);
            continue;
        }

        // Creates the inventory ui slots holder
        create_ui!(Node->slot_holder_node);
        slot_holder_node.style = Style {
            flex_wrap: FlexWrap::Wrap,
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Start,
            width: Val::Percent(100.),
            row_gap: Val::Px(15.0),
            column_gap: Val::Px(15.0),
            height: Val::Auto,
            ..slot_holder_node.style
        };

        let slot_holder_entity = commands.spawn(slot_holder_node).id();
        commands
            .entity(event.ui_inventory_parent)
            .push_children(&[slot_holder_entity]);

        // Creates the inventory so we can know if it is already open or not
        inventory_ui_manager.create_inventory_ui(inventory_entity, slot_holder_entity);

        // Creates the ui slot for each slot in the inventory
        for slot in slots.iter() {
            let Ok(mut slot) = slot_query.get_mut(*slot) else {
                continue;
            };

            create_ui!(Btn->slot_node);
            slot_node.style = Style {
                width: Val::Px(128.),
                height: Val::Px(128.),
                ..slot_node.style
            };
            slot_node.background_color = BackgroundColor(Color::BLACK);

            // If we have an item in that slot, then we must display it
            if let Some(item) = slot.item {
                let Ok(item) = item_query.get(item) else {
                    continue;
                };

                let Some(item_data) = item_database.get_item_data(item) else {
                    continue;
                };

                slot_node.background_color = BackgroundColor(Color::WHITE);
                slot_node.image = UiImage::new(asset_server.load(item_data.asset.clone()));
            }

            let slot_ui_entity = commands.spawn((slot_node, UIInventorySlot)).id();
            commands
                .entity(slot_holder_entity)
                .push_children(&[slot_ui_entity]);

            // This is so we can change the ui slot image according to the item in that slot
            slot.ui_slot = Some(slot_ui_entity);
        }
    }
}

pub fn update_inventory_ui(
    inventory_query: Query<(Entity, &Children), With<Inventory>>,
    slot_query: Query<&InventorySlot, Changed<InventorySlot>>,
    item_query: Query<&Item>,
    inventory_ui_manager: Res<InventoryUIManager>,
    mut ui_slot_query: Query<(&mut UiImage, &mut BackgroundColor), With<UIInventorySlot>>,
    item_database: Res<ItemDatabase>,
    asset_server: Res<AssetServer>,
) {
    for (inv_entity, slots) in inventory_query.iter() {
        // Makes sure that inventory is actually open on the screen
        if !inventory_ui_manager.inventory_has_ui(&inv_entity) {
            continue;
        }

        for slot in slots.iter() {
            let Ok(slot) = slot_query.get(*slot) else {
                continue;
            };

            // If we can find that slot open on the screen then we get the image and color
            let Ok((mut image, mut bg_color)) = ui_slot_query.get_mut(slot.ui_slot.unwrap())
            else {
                continue;
            };

            // If we have an item in that slot, then we must update that slot
            if let Some(item) = slot.item {
                let Ok(item) = item_query.get(item) else {
                    continue;
                };

                let Some(item_data) = item_database.get_item_data(item) else {
                    continue;
                };

                *image = UiImage::new(asset_server.load(item_data.asset.clone()));
                *bg_color = BackgroundColor(Color::WHITE);
                continue;
            }

            // If we don't have an item in that slot then we do not display it
            *bg_color = BackgroundColor(Color::BLACK);
        }
    }
}

pub fn open_inventory(
    keyboard_input: Res<Input<KeyCode>>,
    mut create_inventory_ui_writer: EventWriter<CreateInventoryUI>,
    inventory_query: Query<(Entity, &Inventory), With<Player>>,
    inventory_ui_manager: Res<InventoryUIManager>,
) {
    if keyboard_input.just_pressed(KeyCode::Tab) {
        let entity = inventory_query.single().0;

        info!("Tab");
        create_inventory_ui_writer.send(CreateInventoryUI {
            inventory_entity: entity,
            ui_inventory_parent: inventory_ui_manager.root(),
        })
    }
}
