use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;

pub fn seek(
    mut query: Query<(&Seek, &Transform, &mut Velocity)>
) {
    for (target, transform, mut velocity) in query.iter_mut(){
        let delta = **target - transform.translation.truncate();
        let dist = delta.length() - target.dist;

        velocity.desired += delta.normalize_or_zero() * SPEED;
        if dist < SLOWING_RADIUS { velocity.desired *= dist / SLOWING_RADIUS}
    }
}

pub fn flee (
    mut query: Query<(&Flee, &Transform, &mut Velocity)>
) {
    for (flee, transform, mut velocity) in query.iter_mut(){
        let delta = transform.translation.truncate() - **flee;
        if delta.length() < FLEE_RADIUS {
            velocity.desired += (delta).normalize_or_zero() * SPEED;
        }
    }
}

// Wander
// if let Some(mut wander) = wander {
//     let wander_radius = 50.0;
//     let wander_distance = 100.0;
//     let wander_jitter = 60.0;
//
//     **wander += rng.gen_range(-wander_jitter..=wander_jitter);
//
//     let circle_center = **velocity * wander_distance;
//     let displacement = Vec2 {
//         x: wander_radius * wander.cos(),
//         y: wander_radius * wander.sin(),
//     };
//
//     let wander_target = circle_center + displacement;
//     desired_velocity += (wander_target - current_pos).normalize_or_zero();
//
// }