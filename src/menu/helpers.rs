use crate::common::ui::assets::{ButtonAssets, PanelAssets};
use bevy::prelude::*;

use super::{components::PlayButton, constants::FONT_SIZE};

pub fn create_button_container(
    parent: &mut ChildBuilder,
) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(33.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(30.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            // Play button
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            align_self: AlignSelf::Center,
                            height: Val::Px(100.),
                            width: Val::Px(500.),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        background_color: BackgroundColor(Color::BLACK),
                        ..Default::default()
                    },
                    PlayButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play",
                        TextStyle {
                            font_size: 16.0,
                            ..Default::default()
                        },
                    ));
                });

            // Settings button
            parent
                .spawn((ButtonBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        height: Val::Px(100.),
                        width: Val::Px(500.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::BLACK),
                    ..Default::default()
                },))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Settings",
                        TextStyle {
                            font_size: FONT_SIZE,
                            ..Default::default()
                        },
                    ));
                });

            // Quit button
            parent
                .spawn((ButtonBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        height: Val::Px(100.),
                        width: Val::Px(500.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::BLACK),
                    ..Default::default()
                },))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Quit",
                        TextStyle {
                            font_size: FONT_SIZE,
                            ..Default::default()
                        },
                    ));
                });
        });
}

pub fn create_empty_panel(
    parent: &mut ChildBuilder,
    panel_assets: &Res<PanelAssets>,
    button_assets: &Res<ButtonAssets>,
) {
    parent.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(33.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        ..Default::default()
    });
}

pub fn create_news_panel(
    parent: &mut ChildBuilder,
    panel_assets: &Res<PanelAssets>,
    button_assets: &Res<ButtonAssets>,
) {
    parent.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(33.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        ..Default::default()
    });
}
