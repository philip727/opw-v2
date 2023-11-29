use super::components::Item;

pub struct ItemData {
    pub item: Item,
    pub r#type: ItemType,
}

pub enum ItemType {
    Tool,
    Stackable,
    Placeable,
}
