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
                seek,
                flee,
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
) {
    for (mut velocity, mut transform) in query.iter_mut() {
        let mut steering = velocity.desired - **velocity;
        steering = truncate_exceeded(steering, MAX_FORCE); // /= mass

        **velocity = truncate_exceeded(**velocity + steering, SPEED);
        transform.translation += (**velocity * time.delta_seconds()).extend(0.);
        velocity.desired = Vec2::ZERO;
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
