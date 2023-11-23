use bevy::prelude::*;

use bevy_steering::*;
use bevy_steering::components::{Velocity, Wander};

mod utils;

use utils::{
    spawn_camera,
    spawn_empty_map,
    uvec_pos,
    init_pointer,
    TILE_SIZE
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
        ))
        .init_resource::<utils::CursorPos>()
        .add_plugins(SteeringPlugin)
        .add_systems(Startup, init)
        .add_systems(Update, (
            utils::update_cursor_pos,
            utils::move_player
        ))
        .run();
}

fn init(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    spawn_camera(&mut commands);
    spawn_empty_map(&mut commands, &asset_server);
    init_pointer(&mut commands, &mut meshes, &mut materials);

    for i in 0..1 {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite{
                    color: Color::YELLOW_GREEN,
                    ..default()
                },
                transform: Transform::from_translation((uvec_pos(7+i, 12) + TILE_SIZE / 2.).extend(1.)),
                texture: asset_server.load("player.png"),
                ..default()
            },
            Wander(95.),
            Velocity::new()
        ));
    }
}
