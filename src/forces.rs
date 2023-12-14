use bevy::prelude::*;
use rand::{Rng, thread_rng};
use crate::components::*;
use crate::constants::*;

pub fn seek(
    mut query: Query<(&Seek, &Transform, &mut Velocity)>
) {
    for (target, transform, mut velocity) in query.iter_mut(){
        let delta = **target - transform.translation.truncate();
        let dist = delta.length() - target.dist;

        velocity.change_desired(delta);
        if dist < ARRIVAL_RADIUS { velocity.desired *= dist / ARRIVAL_RADIUS }
    }
}

pub fn flee (
    mut query: Query<(&Flee, &Transform, &mut Velocity)>
) {
    for (flee, transform, mut velocity) in query.iter_mut(){
        let delta = transform.translation.truncate() - **flee;
        if delta.length() < FLEE_RADIUS {
            velocity.change_desired(delta);
        }
    }
}

pub fn avoidance(
    mut query: Query<(Entity, &Transform, &mut Velocity)>,
    neighbours_query: Query<(Entity, &Transform, &Obstacle)>
) {

    for (entity, transform, mut velocity) in query.iter_mut() {
        let position = transform.translation.truncate();
        let ahead = position + velocity.normalize() * MAX_SEE_AHEAD;

        velocity.gizmos.ahead = Some(ahead);

        let ahead2 = position + velocity.normalize() * MAX_SEE_AHEAD * 0.5;
        for (n_entity, n_transform, obstacle) in neighbours_query.iter() {
            if entity != n_entity {
                let n_position = n_transform.translation.truncate();
                for probe in [ahead2, ahead] {
                    if (probe.x - n_position.x).abs() <= obstacle.size && (probe.y - n_position.y).abs() <= obstacle.size {
                        velocity.change_desired(Vec2::new(
                            ahead.x - n_position.x, ahead.y - n_position.y
                        ).normalize() * EVADE_FORCE);
                        break
                    }
                }
            }
        }
    }
}

// pub fn separation(
//     mut query: Query<(Entity, &Transform, &mut Velocity)>,
//     neighbours_query: Query<(Entity, &Velocity)>
// ) {
//
// }

pub fn evade(
    mut query: Query<(&Evade, &Transform, &mut Velocity)>
) {
    for (evade, transform, mut velocity) in query.iter_mut() {
        let dist = (evade.t_pos - transform.translation.truncate()).length();
        let updates_ahead = dist / MAX_SPEED;
        let future_pos = evade.t_pos + evade.t_velocity * updates_ahead;

        // flee logic here
        let delta = transform.translation.truncate() - future_pos;
        if delta.length() < FLEE_RADIUS {
            velocity.change_desired(delta);
        }
    }
}

pub fn pursuit(
    mut query: Query<(&Pursuit, &Transform, &mut Velocity)>
) {
    for (pursuit, transform, mut velocity) in query.iter_mut() {
        let dist = (pursuit.t_pos - transform.translation.truncate()).length();
        let updates_ahead = dist / MAX_SPEED;
        let future_pos = pursuit.t_pos + pursuit.t_velocity * updates_ahead;

        // seek logic here
        let delta = future_pos - transform.translation.truncate();
        velocity.change_desired(delta);
    }
}

pub fn wander (
    mut query: Query<(&mut Wander, &mut Velocity)>
) {
    let mut rng = thread_rng();
    for (mut wander, mut velocity) in query.iter_mut() {
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