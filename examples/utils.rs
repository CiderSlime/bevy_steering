/// NOT FOR LAUNCHING
use bevy::{prelude::*, sprite::Anchor};
use bevy::sprite::MaterialMesh2dBundle;

use bevy_steering::*;
use bevy_steering::components::{Seek, Velocity, Flee};

pub const MAP_SIZE: UVec2 = UVec2::new(24, 24);
pub const TILE_SIZE: Vec2 = Vec2::new(32., 32.);

#[derive(Default, Deref, DerefMut, Resource)]
pub struct CursorPos(pub Option<Vec2>);

#[derive(Resource)]
pub struct PointHandles {
    pub material: Handle<ColorMaterial>,
    pub mesh: Handle<Mesh>,
}

pub fn init_pointer(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>
) {
    let circle_mesh = meshes.add(shape::Circle::new(5.).into());
    let material = materials.add(ColorMaterial::from(Color::GREEN));

    commands.insert_resource(PointHandles {
        mesh: circle_mesh,
        material
    });
}

pub fn update_cursor_pos(
    cameras: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut position: ResMut<CursorPos>,
) {
    let (camera, transform) = cameras.single();
    **position = windows
        .single()
        .cursor_position()
        .and_then(|cursor_pos| camera.viewport_to_world_2d(transform, cursor_pos));
}

pub fn spawn_camera(commands: &mut Commands) {
    commands.spawn(Camera2dBundle {
        // Centering the camera
        transform: Transform::from_translation((MAP_SIZE.as_vec2() * TILE_SIZE / 2.).extend(999.9)),
        ..default()
    });
}

pub fn spawn_empty_map(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let tile_image = asset_server.load("tile.png");
    let uvec_pos = |x: u32, y: u32| UVec2::new(x, y).as_vec2() * TILE_SIZE;
    commands.insert_resource(MapInfo::new(MAP_SIZE, TILE_SIZE));

    for x in 0..MAP_SIZE.x {
        for y in 0..MAP_SIZE.y {
            let pos = uvec_pos(x, y);

            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    anchor: Anchor::BottomLeft,
                    ..default()
                },
                transform: Transform::from_translation(pos.extend(0.)),
                texture: tile_image.clone(),
                ..default()
            });
        }
    }
}

fn set_point(
    mut point: ResMut<Point>,
    mut commands: Commands,
    cursor_pos: Vec2,
    handles: Res<PointHandles>,
) {
    if let Some(id) = **point { commands.entity(id).despawn_recursive() }

    let id = commands.spawn((
        MaterialMesh2dBundle {
            transform: Transform{
                translation: cursor_pos.extend(1.),
                ..default()
            },
            mesh: handles.mesh.clone().into(),
            material: handles.material.clone(),
            ..default()
        },
    )).id();

    **point = Some(id);
}

// Navigate the player to wherever you click
pub fn player_seek(
    mut commands: Commands,
    players: Query<Entity, With<Player>>,
    cursor_pos: Res<CursorPos>,
    mouse: Res<Input<MouseButton>>,
    handles: Res<PointHandles>,
    point: ResMut<Point>

) {
    if mouse.just_pressed(MouseButton::Left) {
        if let Some(cursor_pos) = **cursor_pos {
            commands.entity(players.single()).insert(Seek::new(cursor_pos));

            set_point(point, commands, cursor_pos, handles);
        }
    }
}

pub fn player_flee(
    mut commands: Commands,
    players: Query<Entity, With<Player>>,
    cursor_pos: Res<CursorPos>,
    mouse: Res<Input<MouseButton>>,
    handles: Res<PointHandles>,
    point: ResMut<Point>

) {
    if mouse.just_pressed(MouseButton::Left) {
        if let Some(cursor_pos) = **cursor_pos {
            commands.entity(players.single()).insert(Flee::new(cursor_pos));

            set_point(point, commands, cursor_pos, handles);
        }
    }
}

#[derive(Component)]
pub struct Player;

pub fn uvec_pos(x: u32, y: u32) -> Vec2 {
    UVec2::new(x, y).as_vec2() * TILE_SIZE
}

#[allow(unused)]
pub fn spawn_player(
    commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation((uvec_pos(14, 14) + TILE_SIZE / 2.).extend(1.)),
            texture: asset_server.load("player.png"),
            ..default()
        },
        Player,
        Velocity::new(85.)
    )).id()
}