mod game;
mod systems;
mod main_menu;

use bevy::{prelude::{App, Startup, Query, With, Commands, Transform, ImagePlugin, PluginGroup, Update, Res, AssetServer, Vec2}, window::{Window, PrimaryWindow}, DefaultPlugins, sprite::{SpriteBundle, Sprite}, ecs::schedule::{States, OnEnter, IntoSystemConfigs, common_conditions::in_state}};
use game::GamePlugin;
use main_menu::MainMenuPlugin;

#[derive(States, Clone, Debug, PartialEq, Eq, Hash)]
enum AppState {
    MainMenu,
    InGame,
    Paused
}

impl Default for AppState {
    fn default() -> Self {
        AppState::MainMenu
    }
}

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
    .add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new())
    .add_state::<AppState>()
    .add_plugins(MainMenuPlugin)
    .add_plugins(GamePlugin)
    .add_systems(Startup, systems::spawn_camera)
    .add_systems(OnEnter(AppState::InGame), spawn_rock)
    .add_systems(Update, systems::followup_camera.run_if(in_state(AppState::InGame)))
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

