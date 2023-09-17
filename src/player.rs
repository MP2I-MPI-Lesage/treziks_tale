use bevy::ecs::system::Res;
use bevy::prelude::{Transform, Vec2};
use bevy::sprite::{SpriteBundle, Sprite};
use bevy::{prelude::{Component, With, Commands, Query, AssetServer}, window::{Window, PrimaryWindow}};

#[derive(Component)]
pub struct Player {}

pub fn spawn_player(mut commands: Commands, query: Query<&Window, With<PrimaryWindow>>, asset_server: Res<AssetServer>) {
    let window = query.get_single().unwrap();

    commands.spawn( 
    (
            SpriteBundle {
                transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
                texture: asset_server.load("HeroIdleFrontSprite.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(128.0, 128.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
            Player {},
        )
    );
}
