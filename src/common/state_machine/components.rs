use bevy::{utils::HashMap, prelude::Component};

use super::helpers::State;

#[derive(Component)]
pub struct StateMachine<T: State + Clone> {
    pub states: HashMap<String, T>,
    pub current_state: String,
}

