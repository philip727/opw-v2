use bevy::{
    a11y::{accesskit::NodeBuilder, AccessibilityNode},
    prelude::*,
};

use crate::common::ui::{bundles::ScrollBundle, components::ScrollingRect};

use super::{
    components::{WorldSelectionRoot, WorldsContainer},
    events::SetWorldSelectionRootEvent,
};

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
                    padding: UiRect {
                        left: Val::Px(100.),
                        right: Val::Px(100.),
                        top: Val::Px(100.),
                        bottom: Val::Px(100.),
                    },
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::WHITE),
                ..Default::default()
            })
            .with_children(|parent| {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(55.0),
                            height: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            overflow: Overflow::clip_y(),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent.spawn((
                            ScrollBundle {
                                button: ButtonBundle {
                                    style: Style {
                                        flex_direction: FlexDirection::Column,
                                        align_items: AlignItems::Center,
                                        row_gap: Val::Px(30.0),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },
                                scrolling_rect: ScrollingRect {
                                    sensitivity: 3.,
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            WorldsContainer,
                        ));
                    });

                parent.spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(45.0),
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

pub fn populate_worlds_container(
    mut commands: Commands,
    root_query: Query<Entity, With<WorldsContainer>>,
    mut set_menu_root_event_reader: EventReader<SetWorldSelectionRootEvent>,
) {
    for event in set_menu_root_event_reader.read() {
        let root = root_query.single();

        if !event.visibility {
            // Cleanup, deletes all previous found worlds
            commands.entity(root).clear_children();
            return;
        }

        // Finds all the worlds in the folder
        commands.entity(root).with_children(|parent| {
            for x in 1..10 {
                parent.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.),
                            height: Val::Px(120.),
                            ..Default::default()
                        },
                        background_color: BackgroundColor(Color::BLACK),
                        ..Default::default()
                    },
                    AccessibilityNode(NodeBuilder::new(bevy::a11y::accesskit::Role::ListItem)),
                ));
            }
        });
    }
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

pub fn cleanup_world_selection_ui(
    mut commands: Commands,
    root_query: Query<Entity, With<WorldSelectionRoot>>,
) {
    commands.entity(root_query.single()).despawn();
}
