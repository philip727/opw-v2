use bevy::prelude::*;

use crate::{game::world::events::EnterWorldEvent, states::AppState};

use super::{
    assets::MenuAssets,
    components::{MenuRoot, PlayButton},
    events::SetMenuRootEvent,
};

// Spawns the men root
pub fn spawn_menu_ui(mut commands: Commands, menu_assets: Res<MenuAssets>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                ..Default::default()
            },
            Name::new("Main Menu Root"),
            MenuRoot,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(400.),
                        height: Val::Percent(100.),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::new(
                            Val::Px(0.0),
                            Val::Px(0.0),
                            Val::Px(0.0),
                            Val::Px(0.0),
                        ),
                        row_gap: Val::Px(50.0),
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::rgb_u8(255, 255, 0)),
                    ..Default::default()
                })
                .insert(Name::new("Button Holder"))
                .with_children(|parent| {
                    parent.spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Percent(100.),
                                height: Val::Px(150.),
                                ..Default::default()
                            },
                            background_color: BackgroundColor(Color::rgb_u8(0, 255, 0)),
                            image: UiImage::new(menu_assets.play_button.clone()),
                            ..Default::default()
                        },
                        PlayButton,
                    ));
                    parent.spawn(ButtonBundle {
                        style: Style {
                            width: Val::Percent(100.),
                            height: Val::Px(150.),
                            ..Default::default()
                        },
                        background_color: BackgroundColor(Color::rgb_u8(0, 255, 0)),
                        ..Default::default()
                    });
                    parent.spawn(ButtonBundle {
                        style: Style {
                            width: Val::Percent(100.),
                            height: Val::Px(150.),
                            ..Default::default()
                        },
                        background_color: BackgroundColor(Color::rgb_u8(0, 255, 0)),
                        ..Default::default()
                    });
                });
        });
}

// Makes the menu root invisible
pub fn handle_play_button(
    play_button_query: Query<&Interaction, (With<PlayButton>, Changed<Interaction>)>,
    mut enter_world_event_writer: EventWriter<EnterWorldEvent>,
    mut set_menu_root_event_writer: EventWriter<SetMenuRootEvent>,
) {
    if let Ok(interaction) = play_button_query.get_single() {
        match *interaction {
            Interaction::Pressed => {
                enter_world_event_writer.send(EnterWorldEvent {
                    name: "HAHAHAH".into(),
                    seed: 1204,
                });

                set_menu_root_event_writer.send(SetMenuRootEvent { visibility: false })
            }
            _ => {}
        }
    }
}

// Changes the visibility of the main root
pub fn handle_menu_ui_visibility(
    mut menu_root_query: Query<&mut Visibility, With<MenuRoot>>,
    mut set_menu_root_event_reader: EventReader<SetMenuRootEvent>,
) {
    for event in set_menu_root_event_reader.iter() {
        if let Ok(mut visible) = menu_root_query.get_single_mut() {
            *visible = if event.visibility {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }
}

// Despawns the main root entity
pub fn cleanup_menu_ui(mut commands: Commands, menu_root_query: Query<Entity, With<MenuRoot>>) {
    commands.entity(menu_root_query.single()).despawn();
}

