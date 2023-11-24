use std::default;

use crate::common::ui::assets::{ButtonAssets, PanelAssets};
use bevy::prelude::*;

use super::{components::PlayButton, constants::FONT_SIZE};

#[derive(Clone, PartialEq, Eq, Copy, Default)]
pub enum Page {
    #[default]
    Main,
    WorldSelection,
    Settings,
    Community,
}
