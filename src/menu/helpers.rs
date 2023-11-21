use bevy::prelude::*;

use crate::common::ui::assets::{ButtonAssets, PanelAssets};

use super::{assets::MenuAssets, components::PlayButton, constants::GUI_SCALE};

pub fn create_button_container(parent: &mut ChildBuilder, menu_assets: &Res<MenuAssets>) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(33.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            // Play button
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        height: Val::Px(27. * GUI_SCALE),
                        width: Val::Px(126. * GUI_SCALE),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    image: UiImage::new(menu_assets.play_button.clone()),
                    ..Default::default()
                },
                PlayButton,
            ));

            // Settings button
            parent.spawn((ButtonBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    height: Val::Px(27. * GUI_SCALE),
                    width: Val::Px(126. * GUI_SCALE),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                image: UiImage::new(menu_assets.settings_button.clone()),
                ..Default::default()
            },));

            // Quit button
            parent.spawn((ButtonBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    height: Val::Px(27. * GUI_SCALE),
                    width: Val::Px(126. * GUI_SCALE),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                image: UiImage::new(menu_assets.quit_button.clone()),
                ..Default::default()
            },));
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
