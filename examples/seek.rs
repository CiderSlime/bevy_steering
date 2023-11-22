use bevy::prelude::*;

use bevy_steering::*;

mod utils;

use utils::{
    spawn_camera,
    spawn_empty_map,
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
    spawn_player(&mut commands, &asset_server);
    init_pointer(&mut commands, &mut meshes, &mut materials);
}
