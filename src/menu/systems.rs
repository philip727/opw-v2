use belly::prelude::*;
use bevy::prelude::*;

use super::{components::MenuRoot, events::UpdatePage, helpers::Page, resources::MenuManager};

// Spawns the menu root
pub fn spawn_menu_ui(mut commands: Commands) {
    commands.add(StyleSheet::load("styles/main.ess"));
    commands.add(eml! {
        <body id="main" c:flex-center with=MenuRoot>
            <div s:flex-direction="column">
                <button on:press=|ctx| { ctx.send_event(UpdatePage(Page::WorldSelection))}>"Play"</button>
                <button>"Settings"</button>
                <button>"Quit"</button>
            </div>
        </body>
    })
}

pub fn menu_ui_visibility(mut elements: Elements, menu_manager: Res<MenuManager>) {
    if !menu_manager.is_changed() {
        return;
    }

    match menu_manager.page {
        Page::Main => {
            elements.select("#main").remove_class("hidden");
        }
        _ => {
            elements.select("#main").add_class("hidden");
        }
    }
}

pub fn handle_page_updates(
    mut update_page_event_reader: EventReader<UpdatePage>,
    mut menu_manager: ResMut<MenuManager>,
) {
    for event in update_page_event_reader.read() {
        menu_manager.page = event.0;
    }
}

// Despawns the main root entity
pub fn cleanup_menu_ui(mut commands: Commands, root_query: Query<Entity, With<MenuRoot>>) {
    let Ok(entity) = root_query.get_single() else {
        return;
    };

    commands.entity(entity).despawn();
}
