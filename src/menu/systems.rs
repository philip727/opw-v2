use bevy::prelude::*;

use crate::{
    common::ui::{
        assets::{ButtonAssets, PanelAssets},
        common::helpers::{spawn_ui_button, spawn_ui_text},
    },
    game::world::events::EnterWorldEvent,
};

use super::{
    components::{MenuRoot, PlayButton},
    constants::GUI_SCALE,
    events::SetMenuRootEvent,
};

// Spawns the men root
pub fn spawn_menu_ui(
    mut commands: Commands,
    button_assets: Res<ButtonAssets>,
    panel_assets: Res<PanelAssets>,
) {
    // Main Menu Root
    let mut menu_root = commands.spawn((
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
    ));

    menu_root.with_children(|parent| {
        // Main menu back panel
        parent
            .spawn((
                ImageBundle {
                    style: Style {
                        width: Val::Px(149. * GUI_SCALE),
                        height: Val::Px(237. * GUI_SCALE),
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
                    image: UiImage::new(panel_assets.tall_slim.clone()),
                    ..Default::default()
                },
                Name::new("Button Holder"),
            ))
            .with_children(|parent| {
                // Play button
                parent
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                align_self: AlignSelf::Center,
                                height: Val::Px(22. * GUI_SCALE),
                                width: Val::Px(99. * GUI_SCALE),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            image: UiImage::new(button_assets.outlined_button.clone()),
                            ..Default::default()
                        },
                        PlayButton,
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "Play",
                            TextStyle {
                                font_size: 32.0,
                                ..Default::default()
                            },
                        ));
                    });

                // Quit button
                parent
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                align_self: AlignSelf::Center,
                                height: Val::Px(22. * GUI_SCALE),
                                width: Val::Px(99. * GUI_SCALE),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            image: UiImage::new(button_assets.outlined_button.clone()),
                            ..Default::default()
                        },
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "Quit",
                            TextStyle {
                                font_size: 32.0,
                                ..Default::default()
                            },
                        ));
                    });
            });
    });
}

// Makes the menu root invisible
pub fn handle_play_button(
    mut play_button_query: Query<
        (&Interaction, &mut UiImage, &mut Style),
        (With<PlayButton>, Changed<Interaction>),
    >,
    mut enter_world_event_writer: EventWriter<EnterWorldEvent>,
    mut set_menu_root_event_writer: EventWriter<SetMenuRootEvent>,
    button_assets: Res<ButtonAssets>,
) {
    if let Ok((interaction, mut image, mut style)) = play_button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                enter_world_event_writer.send(EnterWorldEvent {
                    name: "HAHAHAH".into(),
                    seed: 1204,
                });

                set_menu_root_event_writer.send(SetMenuRootEvent { visibility: false })
            }
            Interaction::Hovered => {
                //*image = UiImage::new(menu_assets.play_button_hover.clone());
                //style.height = Val::Px(37. * GUI_SCALE);
            }
            Interaction::None => {
                //style.height = Val::Px(41. * GUI_SCALE);
                *image = UiImage::new(button_assets.outlined_button.clone());
            }
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
