use bevy::prelude::*;

#[derive(Event, PartialEq, Eq, Clone)]
pub struct SetWorldSelectionRootEvent {
    pub visibility: bool
}
