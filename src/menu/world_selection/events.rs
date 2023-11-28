use bevy::prelude::*;

use super::components::WorldSelectionData;

type WorldName = String;
#[derive(Event, PartialEq, Clone)]
pub struct UpdateSelectedWorld(pub WorldName, pub WorldSelectionData);

#[derive(Event, PartialEq, Clone)]
pub struct StartSelectedWorld;
