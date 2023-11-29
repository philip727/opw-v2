use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;

#[derive(Component, Clone, Debug, InspectorOptions)]
pub struct Item {
    pub id: u32,
    pub name: String,
}

#[derive(Component)]
pub struct Stackable;

#[derive(Component)]
pub struct Placeable;

#[derive(Component)]
pub struct Tool;
