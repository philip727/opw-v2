use bevy::{
    a11y::{accesskit::NodeBuilder, AccessibilityNode},
    prelude::*,
};
use std::fs;

use crate::{
    common::ui::{bundles::ScrollBundle, components::ScrollingRect},
    menu::constants::FONT_SIZE,
};

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
    // World Selection Root
    let mut root = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Start,
                    flex_direction: FlexDirection::Column,
                    padding: UiRect {
                        left: Val::Px(50.),
                        right: Val::Px(50.),
                        top: Val::Px(50.),
                        bottom: Val::Px(50.),
                    },
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::rgb_u8(42, 156, 191)),
                visibility: Visibility::Hidden,
                ..Default::default()
            },
            Name::new("World Selection Root"),
            WorldSelectionRoot,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(65.0),
                        padding: UiRect {
                            left: Val::Px(35.),
                            right: Val::Px(35.),
                            top: Val::Px(35.),
                            bottom: Val::Px(35.),
                        },
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::WHITE),
                    ..Default::default()
                })
                .with_children(|parent| {
                    // Left Panel
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

                    // Right Panel
                    parent.spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(45.0),
                            height: Val::Percent(100.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            border: UiRect {
                                left: Val::Px(5.0),
                                right: Val::Px(5.0),
                                top: Val::Px(5.0),
                                bottom: Val::Px(5.0),
                            },
                            ..Default::default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        ..Default::default()
                    });
                });

            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(35.0),
                    margin: UiRect {
                        top: Val::Percent(5.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::WHITE),
                ..Default::default()
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
            match WorldSelectionData::load_all(WORLDS_DIR_PATH) {
                Ok(worlds) => {
                    for (name, data) in worlds {
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: Style {
                                        width: Val::Percent(100.),
                                        height: Val::Px(120.),
                                        padding: UiRect {
                                            top: Val::Px(20.),
                                            left: Val::Px(25.),
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    },
                                    background_color: BackgroundColor(Color::BLACK),
                                    ..Default::default()
                                },
                                AccessibilityNode(NodeBuilder::new(
                                    bevy::a11y::accesskit::Role::ListItem,
                                )),
                                data.clone(),
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    name + " " + &data.seed.to_string(),
                                    TextStyle {
                                        font_size: FONT_SIZE,
                                        ..Default::default()
                                    },
                                ));
                            });
                    }
                }
                Err(e) => {
                    parent.spawn(TextBundle::from_section(
                        e.to_string(),
                        TextStyle {
                            color: Color::BLACK,
                            font_size: FONT_SIZE,
                            ..Default::default()
                        },
                    ));
                }
            }
        });
    }
}

pub fn handle_selecting_world(
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
