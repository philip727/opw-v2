use bevy::prelude::*;

#[derive(Component)]
pub struct MovementController {
    pub speed: f32,
}

#[derive(Component)]
pub struct DirectionController;

impl Default for MovementController {
    fn default() -> Self {
        MovementController { speed: 130.0f32 }
    }
}
