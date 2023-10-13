use bevy::prelude::*;

#[derive(Component)]
pub struct CameraTarget {
    pub priority: u8,
}
