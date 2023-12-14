use bevy::prelude::*;

use bevy_steering::*;
use bevy_steering::components::{Obstacle, Velocity};

mod utils;

use utils::{
    spawn_camera,
    spawn_map,
    spawn_player,
    init_pointer
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
            utils::player_seek,
        ))
        .run();
}

fn init(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    spawn_camera(&mut commands);
    spawn_map(&mut commands, &asset_server, vec![
        (10, 10),  // cell

        (10, 18),  // square
        (10, 19),
        (11, 18),
        (11, 19),

        (17, 4),  // gates
        (17, 5),
        (17, 6),
        (17, 7),
        (17, 8),
        (17, 9),
        (17, 10),
        (17, 12),
        (17, 13),
        (17, 14),
        (17, 15),
        (17, 16),
        (17, 17),
    ]);
    spawn_player(&mut commands, &asset_server);
    init_pointer(&mut commands, &mut meshes, &mut materials);
}
