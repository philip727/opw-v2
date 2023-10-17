use bevy::prelude::*;

#[derive(Debug, States, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
pub enum AppState {
    #[default]
    InMenu,
    InGame,
}
