use std::time::Duration;

use bevy::{utils::HashMap};

use crate::common::state_machine::{components::StateMachine, helpers::State};

use super::helpers::{AnimationState, AnimationStateTitle};

impl StateMachine<AnimationState> {
    pub fn new(
        first_state: AnimationStateTitle,
        states: HashMap<AnimationStateTitle, AnimationState>,
    ) -> Self {
        Self {
            states,
            current_state: first_state,
        }
    }

    // If the state doesn't exist, it will error
    pub fn set_state(&mut self, state: String) -> Result<(), ()> {
        self.states.get(&state).ok_or(())?;
        self.current_state = state;

        Ok(())
    }

    pub fn state(&self) -> &AnimationState {
        self.states.get(&self.current_state).as_ref().unwrap()
    }

    pub fn mut_state(&mut self) -> &mut AnimationState {
        self.states.get_mut(&self.current_state).unwrap()
    }

    pub fn add_state(&mut self, state_name: String, animation: AnimationState) {
        self.states.insert(state_name, animation);
    }

    pub fn is_current_state(&self, state_name: String) -> bool {
        self.current_state == state_name
    }

    pub fn tick(&mut self, tick_by: Duration) -> Option<u8> {
        let state = self.mut_state();

        if state.tick(tick_by) {
            let next_frame = state.next_frame();
            if let Some(state) = state.transition_to() {
                let _ = self.set_state(state);
            }

            return Some(next_frame);
        }

        None
    }
}
