use belly::prelude::*;
use bevy::prelude::*;
use std::{fmt, fs};

use crate::{
    game::world::events::EnterWorld,
    menu::{events::UpdatePage, helpers::Page, resources::MenuManager},
};

use super::{
    components::{WorldSelectionData, WorldSelectionRoot},
    constants::WORLDS_DIR_PATH,
    events::{StartSelectedWorld, UpdateSelectedWorld},
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
        <body id="world-selection" s:flex-direction="column" c:flex-center c:center-panel with=WorldSelectionRoot>
            <div s:width="100%" s:height="90%" s:background-color="#dddddd" c:flex-center s:flex-direction="row">
                <div id="worlds-container" s:width="50%" s:height="100%" s:flex-direction="column" s:align-items="start" />
                <div s:width="50%" s:height="100%" s:flex-direction="column" s:align-items="start" s:padding="10px">
                    <label bind:value=from!(WorldSelectionManager:get_world_name()) s:color="#000000" />
                    <button 
                        on:press=|ctx| {
                            ctx.send_event(StartSelectedWorld);
                        }
                        s:width="70px"
                    >
                        "Play"
                    </button>
                </div>
            </div>
            <div s:width="100%" s:height="10%" s:align-items="start">
                <button>
                    <label value="Create World" />
                </button>
                <button on:press=|ctx| { ctx.send_event(UpdatePage(Page::Main)) }>
                    <label value="Back" />
                </button>
            </div>
        </body>
    })
}

pub(crate) fn populate_worlds(mut elements: Elements, menu_manager: Res<MenuManager>) {
    if !menu_manager.is_changed() {
        return;
    }

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
            <button
                c:world-child
                s:width="100%"
                s:height="75px"
                on:press=move |ctx| {ctx.send_event(UpdateSelectedWorld(name.clone(), world.clone()))}
            >
                <label value={name.clone()} />
            </button>
        });
    }
}

pub fn world_selection_ui_visibility(mut elements: Elements, menu_manager: Res<MenuManager>) {
    if !menu_manager.is_changed() {
        return;
    }

    match menu_manager.page {
        Page::WorldSelection => {
            elements.select("#world-selection").remove_class("hidden");
        }
        _ => {
            elements.select("#world-selection").add_class("hidden");
        }
    }
}

pub fn update_selected_world(
    mut selected_world_event_reader: EventReader<UpdateSelectedWorld>,
    mut world_selection_manager: ResMut<WorldSelectionManager>,
) {
    for event in selected_world_event_reader.read() {
        world_selection_manager.name = Some(event.0.clone());
        world_selection_manager.selected_world = Some(event.1);
    }
}

pub fn start_selected_world(
    mut start_world_event_reader: EventReader<StartSelectedWorld>,
    world_selection_manager: Res<WorldSelectionManager>,
    mut enter_world_event_writer: EventWriter<EnterWorld>,
    mut root_query: Query<&mut Style, With<WorldSelectionRoot>>,
) {
    for _ in start_world_event_reader.read() {
        let Some(world) = world_selection_manager.selected_world else {
            return;
        };

        let Ok(mut style) = root_query.get_single_mut() else {
            return;
        };

        enter_world_event_writer.send(EnterWorld {
            name: world_selection_manager.get_world_name(),
            seed: world.seed,
        });
        style.display = Display::None;
    }
}

pub fn cleanup_world_selection_ui(
    mut commands: Commands,
    mut root_query: Query<Entity, With<WorldSelectionRoot>>,
) {
    let Ok(entity) = root_query.get_single_mut() else {
        return;
    };

    commands.entity(entity).despawn();
}
