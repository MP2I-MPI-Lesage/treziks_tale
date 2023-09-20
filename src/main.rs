mod player;

use bevy::{prelude::{App, Startup, Query, With, Commands, Camera2dBundle, Transform, ImagePlugin, PluginGroup, Update}, window::{Window, PrimaryWindow}, utils::default, DefaultPlugins};


fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
    .add_systems(Startup, spawn_camera)
    .add_systems(Update, player::player_movements)
    .add_systems(Startup, player::spawn_player)
    .run()

}

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        }
    );
}