use bevy::{
    a11y::{accesskit::NodeBuilder, AccessibilityNode},
    prelude::*,
};

use super::components::ScrollingRect;

#[derive(Bundle)]
pub struct ScrollBundle {
    pub button: ButtonBundle,
    pub accessibility: AccessibilityNode,
    pub scrolling_rect: ScrollingRect,
}

impl Default for ScrollBundle {
    fn default() -> Self {
        ScrollBundle {
            button: ButtonBundle::default(),
            accessibility: AccessibilityNode(NodeBuilder::new(bevy::a11y::accesskit::Role::List)),
            scrolling_rect: ScrollingRect::default(),
        }
    }
}
