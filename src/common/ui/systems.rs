use bevy::{input::mouse::MouseWheel, prelude::*};

use super::components::ScrollingRect;

pub fn handle_scrolling_rect_focus(
    mut interaction_query: Query<
        (&Interaction, &mut ScrollingRect),
        (Changed<Interaction>, With<ScrollingRect>),
    >,
) {
    for (interaction, mut rect) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Hovered => rect.active = true,
            Interaction::None => rect.active = false,
            _ => {}
        }
    }
}

pub fn handle_scrolling_rect_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut list_query: Query<(&mut ScrollingRect, &mut Style, &Parent, &Node)>,
    node_query: Query<&Node>,
) {
    for event in mouse_wheel_events.read() {
        for (mut rect, mut style, parent, nodes) in &mut list_query {
            // If it's not focused then don't scroll
            if !rect.active {
                continue;
            }

            let items_height = nodes.size().y;
            let container_height = node_query.get(parent.get()).unwrap().size().y;

            let max_scroll = (items_height - container_height).max(0.);

            let dy = match event.unit {
                bevy::input::mouse::MouseScrollUnit::Line => event.y * 20.,
                bevy::input::mouse::MouseScrollUnit::Pixel => event.y,
            };

            rect.position += dy * rect.sensitivity;
            rect.position = rect.position.clamp(-max_scroll, 0.);
            style.top = Val::Px(rect.position);
        }
    }
}
