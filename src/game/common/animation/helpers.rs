use std::time::Duration;

use bevy::prelude::*;

use crate::common::state_machine::helpers::State;

pub type AnimationStateTitle = String;
#[derive(Clone, PartialEq, Eq)]
pub struct AnimationState {
    pub frames: Vec<u8>,
    current_index: usize,
    pub timer: Timer,
}

impl State for AnimationState {
    fn r#loop(&self) -> bool {
        false
    }

    fn timer(&self) -> &Timer {
        &self.timer
    }

    fn mut_timer(&mut self) -> &mut Timer {
        &mut self.timer
    }

    fn transition_to(&self) -> Option<String> {
        None
    }
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

    pub fn next_frame(&mut self) -> u8 {
        if self.current_index >= self.frames.len() - 1 {
            self.current_index = 0;
        } else {
            self.current_index += 1;
        }

        self.frames[self.current_index]
    }
}
