use image::io::Reader as ImageReader;
use std::{fs, path::PathBuf};

use bevy::{prelude::*, sprite::TextureAtlas, utils::HashMap};
use serde::{Deserialize, Serialize};

use crate::game::common::animation::{components::AnimationStateMachine, helpers::{AnimationState, AnimationStateTitle}};

pub type EntitySkinId = String;
#[derive(Serialize, Deserialize, Clone)]
pub struct EntitySkin {
    pub id: EntitySkinId,
    pub asset_location: String,
    pub sprite_size: [u32; 2],
    pub animations: Vec<EntitySkinAnimation>,
    default_state: String,
}

impl EntitySkin {
    /// Loads the skin data from a directory. [path] must be a directory.
    pub fn load_from_path(path: PathBuf) -> anyhow::Result<EntitySkin> {
        let skin_data_path = path.to_str().unwrap().to_owned() + "/data.json";
        let _skin_dir_name = path.file_name().unwrap().to_str().unwrap().to_owned();

        let file_string = fs::read_to_string(&skin_data_path)?;

        let skin_data = serde_json::from_str::<EntitySkin>(&file_string)?;

        Ok(skin_data)
    }

    pub fn generate_texture_atlas(&self, asset_server: &AssetServer) -> TextureAtlas {
        let texture_handle: Handle<Image> = asset_server.load(&self.asset_location);
        let total_path = "./assets/".to_owned() + &self.asset_location;
        // Calculate cols and rows
        let image = ImageReader::open(&total_path)
            .expect(&format!("Failed to load \"{}\"", total_path))
            .decode()
            .expect(&format!("Failed to load \"{}\"", total_path));

        let cols = (image.width() / self.sprite_size[0]) as usize;
        let rows = (image.height() / self.sprite_size[1]) as usize;

        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(self.sprite_size[0] as f32, self.sprite_size[1] as f32),
            cols,
            rows,
            None,
            None,
        );

        texture_atlas
    }
}

impl Into<AnimationStateMachine> for EntitySkin {
    fn into(self) -> AnimationStateMachine {
        let mut state_hashmap = HashMap::new();

        for animation in self.animations {
            state_hashmap.insert(
                animation.title,
                AnimationState::new(animation.frames, animation.animation_length),
            );
        }

        AnimationStateMachine::new(self.default_state, state_hashmap)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EntitySkinAnimation {
    pub title: AnimationStateTitle,
    pub frames: Vec<u8>,
    pub animation_length: f32,
}
