use bevy::prelude::*;

#[derive(Debug, Event, PartialEq, Eq, Ord, PartialOrd, Hash, Clone)]
pub struct EnterWorldEvent {
    pub name: String,
    pub seed: u32,
}
