use bevy::prelude::*;

#[macro_export]
macro_rules! create_ui {
    (Btn->$name:ident) => {
        let mut $name = ButtonBundle {
            ..Default::default()
        };
    };
    (Node->$name:ident) => {
        let mut $name = NodeBundle {
            ..Default::default()
        };
    };
    (NodeCenter->$name:ident) => {
        let mut $name = NodeBundle {
            style: Style {
                justify_items: JustifyItems::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            ..Default::default()
        };
    };
    (Grid->$name:ident) => {
        let mut $name = NodeBundle {
            style: Style {
                display: Display::Grid,
                ..Default::default()
            },
            ..Default::default()
        };
    };
    (GridCenter->$name:ident) => {
        let mut $name = NodeBundle {
            style: Style {
                display: Display::Grid,
                justify_items: JustifyItems::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            ..Default::default()
        };
    };
    (Col->$name:ident) => {
        let mut $name = NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            ..Default::default()
        };
    };
}
