use bevy::prelude::*;

pub fn spawn_menu_ui(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("Menu Root"))
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
