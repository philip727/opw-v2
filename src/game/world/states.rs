use bevy::prelude::*;

#[derive(States, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum WorldState {
    #[default]
    Initial,
    LoadBiomes,
    GenerateTextureAtlas,
    LoadItems,
    LoadPlayer,
    Created,
}
