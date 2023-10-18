use bevy::prelude::*;

#[derive(Event, PartialEq, Eq, Clone)]
pub struct SetMenuRootEvent {
    pub visibility: bool
}
