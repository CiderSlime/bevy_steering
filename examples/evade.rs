use bevy::prelude::*;

use bevy_steering::*;

use bevy_steering::components::{Evade, Velocity};

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
    let player = spawn_player(&mut commands, &asset_server);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite{
                color: Color::YELLOW_GREEN,
                ..default()
            },
            transform: Transform::from_translation((uvec_pos(10, 10) + TILE_SIZE / 2.).extend(1.)),
            texture: asset_server.load("player.png"),
            ..default()
        },
        Evade::new(player),
        Velocity::new()
    ));

}
