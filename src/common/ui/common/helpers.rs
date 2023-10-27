use bevy::{ecs::system::EntityCommands, prelude::*};

pub fn spawn_ui_button(
    parent: &mut EntityCommands,
    style: Style,
    tag: impl Component,
    image: Option<UiImage>,
    color: Option<BackgroundColor>,
) {
    parent.with_children(|parent| {
        parent.spawn((
            ButtonBundle {
                style,
                image: image.unwrap_or(UiImage::default()),
                background_color: color.unwrap_or(BackgroundColor(Color::WHITE)),
                ..Default::default()
            },
            tag,
        ));
    });
}
