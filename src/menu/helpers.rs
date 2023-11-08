use bevy::prelude::*;

use crate::common::ui::assets::{ButtonAssets, PanelAssets};

use super::{components::PlayButton, constants::GUI_SCALE};

pub fn create_button_container(
    parent: &mut ChildBuilder,
    panel_assets: &Res<PanelAssets>,
    button_assets: &Res<ButtonAssets>,
) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(33.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
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
                        .spawn((ButtonBundle {
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
                        },))
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

pub fn create_left_panel(
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
