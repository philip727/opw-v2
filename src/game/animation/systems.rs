use bevy::prelude::*;

use super::components::AnimationStateMachine;

pub fn handle_animation_machines(
    mut sprite_query: Query<(&mut AnimationStateMachine, &mut TextureAtlasSprite)>,
    time: Res<Time>,
) {
    for (mut animation, mut sprite) in sprite_query.iter_mut() {
        let state = animation.mut_state();
        state.timer.tick(time.delta());

        if state.timer.just_finished() {
            sprite.index = state.next() as usize;
        }
    }
}
