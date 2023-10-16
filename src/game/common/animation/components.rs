use bevy::{prelude::*, utils::HashMap};

use super::helpers::{AnimationState, AnimationStateTitle};

#[derive(Component)]
pub struct AnimationStateMachine {
    states: HashMap<AnimationStateTitle, AnimationState>,
    current_state: AnimationStateTitle,
}

impl AnimationStateMachine {
    pub fn new(
        first_state: AnimationStateTitle,
        states: HashMap<AnimationStateTitle, AnimationState>,
    ) -> AnimationStateMachine {
        AnimationStateMachine {
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
}
