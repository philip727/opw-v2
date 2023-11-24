use anyhow::Ok;
use belly::prelude::*;
use bevy::{
    a11y::{accesskit::NodeBuilder, AccessibilityNode},
    prelude::*,
};
use std::fs;

use crate::menu::{constants::FONT_SIZE, helpers::Page, resources::MenuManager};

use super::{
    components::{WorldSelectionData, WorldSelectionRoot, WorldsContainer},
    constants::WORLDS_DIR_PATH,
    events::SetWorldSelectionRootEvent,
    resources::WorldSelectionManager,
};

pub fn create_saves_directory() {
    if !fs::metadata(WORLDS_DIR_PATH).is_ok() || !fs::metadata(WORLDS_DIR_PATH).unwrap().is_dir() {
        fs::create_dir(WORLDS_DIR_PATH).expect(&format!(
            "Failed to create \"{}\"  directory.",
            WORLDS_DIR_PATH
        ))
    }
}

pub fn spawn_world_selection_ui(mut commands: Commands) {
    commands.add(StyleSheet::load("styles/main.ess"));
    commands.add(StyleSheet::load("styles/world_selection.ess"));
    commands.add(eml! {
        <body id="world-selection" c:flex-center c:center-panel with=WorldSelectionRoot>
            <div s:width="100%" s:height="100%" s:background-color="#dddddd" c:flex-center s:flex-direction="row">
                <div id="worlds-container" s:width="50%" s:height="100%" s:flex-direction="column" s:align-items="start">

                </div>
                <div s:width="50%">

                </div>
            </div>
        </body>
    })
}

pub(crate) fn populate_worlds(mut elements: Elements, menu_manager: Res<MenuManager>) {
    if menu_manager.is_changed() {
        if menu_manager.page != Page::WorldSelection {
            // Clear worlds
            for entity in elements.select(".world-child").entities() {
                elements.entity(entity).remove();
            }
            return;
        }

        let worlds = WorldSelectionData::load_all(WORLDS_DIR_PATH);

        if let Err(e) = worlds {
            elements.select("#worlds-container").add_child(eml! {
                <label>{e.to_string()}</label>
            });

            return;
        }

        for (name, world) in worlds.unwrap() {
            elements.select("#worlds-container").add_child(eml! {
                <button c:world-child s:width="100%" s:height="75px">
                    <label value={name} />
                </button>
            });
        }
    }
}

pub fn world_selection_ui_visibility(mut elements: Elements, menu_manager: Res<MenuManager>) {
    if menu_manager.is_changed() {
        match menu_manager.page {
            Page::WorldSelection => {
                elements.select("#world-selection").remove_class("hidden");
            }
            _ => {
                elements.select("#world-selection").add_class("hidden");
            }
        }
    }
}

pub fn current_selected_world_display(
    button_query: Query<(&WorldSelectionData, &Interaction), Changed<Interaction>>,
    mut world_selection_manager: ResMut<WorldSelectionManager>,
) {
    for (selected_world, interaction) in button_query.iter() {
        if matches!(interaction, Interaction::Pressed) {
            world_selection_manager.selected_world = Some(selected_world.clone());

            info!("{:?}", world_selection_manager.selected_world);
        }
    }
}

pub fn cleanup_world_selection_ui(
    mut commands: Commands,
    root_query: Query<Entity, With<WorldSelectionRoot>>,
) {
    commands.entity(root_query.single()).despawn();
}
