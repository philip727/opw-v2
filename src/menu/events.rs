use bevy::prelude::*;

use super::helpers::Page;

#[derive(Event, PartialEq, Eq, Clone)]
#[repr(transparent)]
pub struct UpdatePage(pub Page);
