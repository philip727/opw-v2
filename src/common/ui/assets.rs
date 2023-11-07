use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct ButtonAssets {
    #[asset(path="ui/button/outlined-button.png")]
    pub outlined_button: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct PanelAssets {
    #[asset(path="ui/panels/tall-slim-panel.png")]
    pub tall_slim: Handle<Image>
}
