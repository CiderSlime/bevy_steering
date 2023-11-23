use bevy::prelude::*;
use rand::{prelude, Rng, thread_rng};
use crate::components::*;
use crate::constants::*;

pub fn seek(
    mut query: Query<(&Seek, &Transform, &mut Velocity)>
) {
    for (target, transform, mut velocity) in query.iter_mut(){
        let delta = **target - transform.translation.truncate();
        let dist = delta.length() - target.dist;

        velocity.desired += delta.normalize_or_zero() * SPEED;
        if dist < ARRIVAL_RADIUS { velocity.desired *= dist / ARRIVAL_RADIUS }
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

pub fn wander (
    mut query: Query<(&mut Wander, &Transform, &mut Velocity)>
) {
    let mut rng = thread_rng();
    for (mut wander, transform, mut velocity) in query.iter_mut() {
        let circle_center = velocity.normalize_or_zero() * CIRCLE_DISTANCE;
        let mut displacement = Vec2::new(0., -CIRCLE_DISTANCE);

        // set angle
        let len = displacement.length();
        displacement.x = wander.cos() * len;
        displacement.y = wander.sin() * len;

        **wander += rng.gen_range(-ANGLE_CHANGE..ANGLE_CHANGE);
        velocity.desired += circle_center + displacement;
    }
}