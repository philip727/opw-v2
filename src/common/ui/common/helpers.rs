use bevy::{ecs::system::EntityCommands, prelude::*};

pub fn spawn_ui_button(
    parent: &mut EntityCommands,
    style: Style,
    tag: impl Component,
    image: Option<UiImage>,
    color: Option<BackgroundColor>,
) -> Option<Entity> {
    let mut button_entity = None;
    parent.with_children(|parent| {
        button_entity = Some(
            parent
                .spawn((
                    ButtonBundle {
                        style,
                        image: image.unwrap_or(UiImage::default()),
                        background_color: color.unwrap_or(BackgroundColor(Color::WHITE)),
                        ..Default::default()
                    },
                    tag,
                ))
                .id(),
        );
    });

    button_entity
}

pub fn spawn_ui_text(
    parent: &mut EntityCommands,
    style: TextStyle,
    tag: Option<impl Component>,
    text: impl Into<String>,
) -> Option<Entity> {
    let mut text_entity = None;
    parent.with_children(|parent| {
        text_entity = Some(if let Some(tag) = tag {
            parent
                .spawn(TextBundle::from_section(text, style))
                .insert(tag)
                .id()
        } else {
            parent.spawn(TextBundle::from_section(text, style)).id()
        })
    });

    text_entity
}
