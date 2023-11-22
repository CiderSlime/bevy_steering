mod common;
pub mod components;

use bevy::prelude::*;
use rand::{Rng, thread_rng};

use common::MovementSet;
use components::*;

pub struct SteeringPlugin;

const MAX_FORCE: f32 = 4.;
const SPEED: f32 = 60.;
const FLEE_RADIUS: f32 = 100.;
const SLOWING_RADIUS: f32 = 38.;
const WANDER_DEGREE: f64 = 1.;
const WANDER_FORCE: f32 = 0.0024;
const MAX_SEE_AHEAD: f32 = 82.;
const EVADE_FORCE: f32 = 8.0;


#[derive(Resource, Deref, DerefMut, Default)]
pub struct Point(Option<Entity>);

#[derive(Resource)]
pub struct MapInfo {
    map_size: UVec2,
    tile_size: Vec2,
    map_width: f32,
    map_height: f32,
} impl MapInfo {
    pub fn new(map_size: UVec2, tile_size: Vec2) -> Self {
        MapInfo{
            map_size,
            tile_size,
            map_width: map_size.x as f32 * tile_size.x,
            map_height: map_size.y as f32 * tile_size.y
        }
    }
}

fn prepare_context(
    mut actors: Query<&mut Velocity>,
) {
    for mut velocity in actors.iter_mut() {
        // let mut current_pos = transform.translation.truncate();
        velocity.desired = Vec2::ZERO;


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

        // Evade map borders
        // if velocity.is_some() {
        //     let ahead = current_pos + **velocity * speed * MAX_SEE_AHEAD;
        //     let mut evade = Vec2::ZERO;
        //
        //     // collide map borders
        //     if ahead.x < 0. {
        //         evade.x = -ahead.x;
        //     }
        //     else if ahead.x > info.map_width {
        //         evade.x = info.map_width - ahead.x;
        //     }
        //
        //     if ahead.y < 0. {
        //         evade.y = -ahead.y;
        //     }
        //     else if ahead.y > info.map_height {
        //         evade.y = info.map_height - ahead.y;
        //     }
        //
        //     desired_velocity += evade * EVADE_FORCE;
        // }
    }
}

fn seek(
    mut query: Query<(&Seek, &Transform, &mut Velocity)>
) {
        for (target, transform, mut velocity) in query.iter_mut(){
        let delta = **target - transform.translation.truncate();
        let dist = delta.length() - target.dist;

        // if dist < 10.0 {
            // if let Some(id) = **point {
            //     commands.entity(id).despawn_recursive();
            //     **point = None;
            // }
            // commands.entity(entity).remove::<MoveTo>();
            // **velocity = Vec2::ZERO;
            // continue
        // }

        velocity.desired += delta.normalize_or_zero() * SPEED;
        if dist < SLOWING_RADIUS { velocity.desired *= dist / SLOWING_RADIUS}
    }
}

// fn flee (
//     flee: Option<&Flee>,
//     current_pos: &Vec2,
//     speed: &mut f32,
//     desired_velocity: &mut Vec2
//
// ) {
//     if let Some(flee) = flee {
//         let delta = *current_pos - **flee;
//         if delta.length() < FLEE_RADIUS {
//             *desired_velocity += (delta).normalize_or_zero();
//         } else if target.is_none() && wander.is_none() {
//             **velocity  = Vec2::ZERO;
//         }
//     }
// }

fn finalize(
    mut query: Query<(&mut Velocity, &mut Transform)>,
    time: Res<Time>,
) {
    for (mut velocity, mut transform) in query.iter_mut() {
        let mut steering = velocity.desired - **velocity;
        steering = truncate_exceeded(steering, MAX_FORCE); // /= mass

        **velocity = truncate_exceeded(**velocity + steering, SPEED);
        transform.translation += (**velocity * time.delta_seconds()).extend(0.);
    }
}

fn get_transforms(
    transforms: Query<&Transform>,
    mut flee: Query<&mut Flee>
) {
    // get entity transforms here to avoid conflicting queries
    for mut flee in flee.iter_mut() {
        let chaser_transform = transforms.get(flee.target).unwrap();
        **flee = chaser_transform.translation.truncate();
    }
}

#[allow(unused)]
fn rotate(v: Vec2, angle_degrees: f64) -> Vec2 {
        let angle_radians = angle_degrees.to_radians();
        let sin_a = angle_radians.sin() as f32;
        let cos_a = angle_radians.cos() as f32;

        let result = Vec2 {
            x: v.x * cos_a - v.y * sin_a,
            y: v.x * sin_a + v.y * cos_a,
        };
        // info!("{} * {} = {}", v, angle_degrees, result);
        result
    }

fn truncate_exceeded(v: Vec2, max: f32) -> Vec2 {
    // truncates Vec2 if it exceeds max length
    let i = max / v.length();
    let i = if i < 1.0 { i } else { 1.0 };
    v * i
}

impl Plugin for SteeringPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                get_transforms,
                prepare_context,
                seek,
                finalize
            ).chain())
            .init_resource::<Point>();
    }
}
