use std::time::Duration;

use bevy::prelude::*;

pub type AnimationStateTitle = String;
#[derive(Clone)]
pub struct AnimationState {
    pub frames: Vec<u8>,
    current_index: usize,
    pub timer: Timer,
}

impl AnimationState {
    pub fn new(frames: Vec<u8>, animation_length: f32) -> AnimationState {
        let time_between_frames = animation_length / frames.len() as f32;
        AnimationState {
            frames,
            current_index: 0,
            timer: Timer::new(
                Duration::from_secs_f32(time_between_frames),
                TimerMode::Repeating,
            ),
        }
    }

    pub fn next(&mut self) -> u8 {
        if self.current_index >= self.frames.len() - 1 {
            self.current_index = 0;
        } else {
            self.current_index += 1;
        }

        self.frames[self.current_index]
    }
}
