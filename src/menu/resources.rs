use bevy::prelude::*;

use super::helpers::Page;

#[derive(Resource, Default)]
pub struct MenuManager {
    pub page: Page
}
