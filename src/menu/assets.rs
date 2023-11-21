use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct MenuAssets {
    #[asset(path="ui/button/menu/play-button.png")]
    pub play_button: Handle<Image>,
    #[asset(path="ui/button/menu/settings-button.png")]
    pub settings_button: Handle<Image>,
    #[asset(path="ui/button/menu/quit-button.png")]
    pub quit_button: Handle<Image>,
}
