use bevy::prelude::*;

use bevy_steering::*;

use bevy_steering::components::{Evade, Pursuit, Velocity, Wander};

mod utils;

use utils::{
    spawn_camera,
    spawn_empty_map,
    spawn_player,
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
            utils::player_seek
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
    spawn_player(&mut commands, &asset_server);

    let target = commands.spawn((
        SpriteBundle {
            sprite: Sprite{
                color: Color::OLIVE,
                ..default()
            },
            transform: Transform::from_translation((uvec_pos(10, 10) + TILE_SIZE / 2.).extend(1.)),
            texture: asset_server.load("player.png"),
            ..default()
        },
        Wander(31.),
        Velocity::new(95.)
    )).id();

    let pursuer = commands.spawn((
        SpriteBundle {
            sprite: Sprite{
                color: Color::RED,
                ..default()
            },
            transform: Transform::from_translation((uvec_pos(1, 1) + TILE_SIZE / 2.).extend(1.)),
            texture: asset_server.load("player.png"),
            ..default()
        },
        Pursuit::new(target),
        Velocity::new(60.)
    )).id();

    commands.entity(target).insert(Evade::new(pursuer));

}
