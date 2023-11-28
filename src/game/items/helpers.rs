use std::fmt::Debug;

#[derive(Clone, Debug, bevy_inspector_egui::InspectorOptions)]
pub struct Item {
    pub id: u32,
    pub name: String,
}
