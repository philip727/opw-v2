use bevy::prelude::*;

#[derive(Component)]
pub struct ScrollingRect {
    pub position: f32,
    pub active: bool,
    pub sensitivity: f32,
}

impl Default for ScrollingRect {
    fn default() -> Self {
        ScrollingRect {
            position: 0.,
            active: false,
            sensitivity: 1.,
        }
    }
}
