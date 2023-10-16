use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity {
    pub displacement: Vec3,
    pub previous_translation: Vec3,
}

impl Velocity {
    pub fn new(pos: Vec3) -> Self {
        Self {
            displacement: Vec3::ZERO,
            previous_translation: pos,
        }
    }
}
