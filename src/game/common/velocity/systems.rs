use bevy::prelude::*;

use super::components::Velocity;

pub fn calculate_displacement(mut velocity_query: Query<(&Transform, &mut Velocity)>) {
    for (transform, mut velocity) in velocity_query.iter_mut() {
        let current_translation = transform.translation;

        let displacement = current_translation - velocity.previous_translation;

        velocity.displacement = displacement;
        velocity.previous_translation = current_translation;
    }
}
