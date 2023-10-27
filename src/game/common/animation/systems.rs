use bevy::prelude::*;

use crate::common::state_machine::{components::StateMachine, helpers::State};

use super::helpers::AnimationState;

pub fn handle_animation_machines(
    mut sprite_query: Query<(&mut StateMachine<AnimationState>, &mut TextureAtlasSprite)>,
    time: Res<Time>,
) {
    for (mut animation, mut sprite) in sprite_query.iter_mut() {
        if let Some(next_frame) = animation.tick(time.delta()) {
            sprite.index = next_frame as usize;
        }
    }
}
