mod player;

use bevy::{prelude::{App, Startup, Query, With, Commands, Camera2dBundle, Transform, ImagePlugin, PluginGroup, Update, Res, AssetServer, Vec2, Component, OrthographicProjection, Camera2d, Camera, GlobalTransform, Without}, window::{Window, PrimaryWindow}, utils::default, DefaultPlugins, transform, sprite::{SpriteBundle, Sprite}, render::camera};
use player::Player;


fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
    .add_systems(Startup, spawn_camera)
    .add_systems(Startup, spawn_rock)
    .add_systems(Update, player::player_movements)
    .add_systems(Startup, player::spawn_player)
    .add_systems(Update, followup_camera)
    .run()

}

fn spawn_rock(mut commands: Commands, query: Query<&Window, With<PrimaryWindow>>, asset_server: Res<AssetServer>) {
    let window = query.get_single().unwrap();

    commands.spawn( 
    (
            SpriteBundle {
                transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
                texture: asset_server.load("rock.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(96.0, 96.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
        )
    );
}

fn followup_camera(p_query: Query<&Transform, (With<Player>, Without<PlayerCamera>)>, mut c_query: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>) { 
    let p_transform = p_query.get_single().unwrap();
    let mut _c_transform = c_query.get_single_mut().unwrap(); 
    
    _c_transform.translation = p_transform.translation
}

#[derive(Component)]
struct PlayerCamera;

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap(); 

    commands.spawn(
        (
            Camera2dBundle {
                transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0), 
                ..default()
            },
            PlayerCamera,
        )
    );
}