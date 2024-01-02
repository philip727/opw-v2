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
    (Grid->$name:ident) => {
        let mut $name = NodeBundle {
            style: Style {
                display: Display::Grid,
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
