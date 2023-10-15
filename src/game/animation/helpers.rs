use std::time::Duration;

use bevy::prelude::*;

pub type AnimationStateTitle = String;
#[derive(Clone)]
pub struct AnimationState {
    pub textures: Vec<u8>,
    current_index: usize,
    pub timer: Timer,
}

impl AnimationState {
    pub fn new(textures: Vec<u8>, animation_length: f32) -> AnimationState {
        let time_between_frames = animation_length / textures.len() as f32;
        AnimationState {
            textures,
            current_index: 0,
            timer: Timer::new(
                Duration::from_secs_f32(time_between_frames),
                TimerMode::Repeating,
            ),
        }
    }

    pub fn next(&mut self) -> u8 {
        if self.current_index >= self.textures.len() - 1 {
            self.current_index = 0;
        } else {
            self.current_index += 1;
        }

        self.textures[self.current_index]
    }
}
