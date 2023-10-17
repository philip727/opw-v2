use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;

#[derive(Component, InspectorOptions, Reflect)]
#[reflect(Component)]
pub struct MovementController {
    #[inspector(min = 0.0)]
    pub speed: f32,
}


impl Default for MovementController {
    fn default() -> Self {
        MovementController { speed: 130.0f32 }
    }
}

#[derive(Component)]
pub struct DirectionController;

#[derive(Component)]
pub struct Player;
