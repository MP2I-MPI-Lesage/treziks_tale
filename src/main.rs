mod player;
mod camera;

use player::actions::PlayerActions;
use bevy::{prelude::{App, Startup, Query, With, Commands, Transform, ImagePlugin, PluginGroup, Update, Res, AssetServer, Vec2}, window::{Window, PrimaryWindow}, DefaultPlugins, sprite::{SpriteBundle, Sprite}, render::{RenderPlugin, settings::{WgpuSettings, Backends}}};

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
    .add_plugins(player::PlayerPlugin)
    .add_systems(Startup, camera::spawn_camera)
    .add_systems(Startup, spawn_rock)
    .add_systems(Update, camera::followup_camera)
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
                    custom_size: Some(Vec2::new(32.0, 32.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
        )
    );
}

