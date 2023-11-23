mod common;
pub mod components;
mod forces;
mod constants;

use bevy::prelude::*;

use common::*;
use components::*;
use forces::*;
use constants::*;

pub struct SteeringPlugin;

impl Plugin for SteeringPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                get_transforms,
                (
                    seek,
                    flee,
                    wander,
                    evade,
                    pursuit
                ),
                finalize
            ).chain())
            .init_resource::<Point>();
    }
}

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

fn finalize(
    mut query: Query<(&mut Velocity, &mut Transform)>,
    time: Res<Time>,
    info: Res<MapInfo>
) {
    for (mut velocity, mut transform) in query.iter_mut() {
        let mut steering = velocity.desired - **velocity;
        steering = truncate_exceeded(steering, MAX_FORCE); // /= mass

        **velocity = truncate_exceeded(**velocity + steering, velocity.speed);
        transform.translation += (**velocity * time.delta_seconds()).extend(0.);
        transform.translation = common::check_overflow(
            transform.translation,
            info.map_width,
            info.map_height
        );
        velocity.desired = Vec2::ZERO;  // should be recalculated during next step
    }
}

fn get_transforms(
    transforms: Query<(&Transform, &Velocity)>,
    mut evade: Query<&mut Evade>,
    mut pursuit: Query<&mut Pursuit>
) {
    // get entity transforms here to avoid conflicting queries
    for mut evade in evade.iter_mut() {
        let (chaser_transform, chaser_velocity) = transforms.get(evade.target).unwrap();
        evade.t_pos = chaser_transform.translation.truncate();
        evade.t_velocity = **chaser_velocity;
    }

    for mut pursuit in pursuit.iter_mut() {
        let (chaser_transform, chaser_velocity) = transforms.get(pursuit.target).unwrap();
        pursuit.t_pos = chaser_transform.translation.truncate();
        pursuit.t_velocity = **chaser_velocity;
    }
}
