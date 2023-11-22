use bevy::prelude::*;

use super::{components::WorldSelectionRoot, events::SetWorldSelectionRootEvent};

pub fn spawn_world_selection_ui(mut commands: Commands) {
    // World Selection Root
    let mut root = commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            background_color: BackgroundColor(Color::rgb_u8(42, 156, 191)),
            visibility: Visibility::Hidden,
            ..Default::default()
        },
        Name::new("World Selection Root"),
        WorldSelectionRoot,
    ));

    root.with_children(|parent| {
        parent
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(90.0),
                    height: Val::Percent(90.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::BLACK),
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(70.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                });

                parent.spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(30.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                });
            });
    });
}



// Changes the visibility of the world selection root
pub fn handle_world_selection_ui_visibility(
    mut menu_root_query: Query<&mut Visibility, With<WorldSelectionRoot>>,
    mut set_menu_root_event_reader: EventReader<SetWorldSelectionRootEvent>,
) {
    for event in set_menu_root_event_reader.read() {
        if let Ok(mut visible) = menu_root_query.get_single_mut() {
            *visible = if event.visibility {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }
}

pub fn cleanup_world_selection_ui(mut commands: Commands, root_query: Query<Entity, With<WorldSelectionRoot>>) {
    commands.entity(root_query.single()).despawn();
}
