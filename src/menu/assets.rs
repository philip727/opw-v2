use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct MenuAssets {
    #[asset(path="ui/menu/play-button.png")]
    pub play_button: Handle<Image>,
    #[asset(path="ui/menu/play-button-hovered.png")]
    pub play_button_hover: Handle<Image>,
}
