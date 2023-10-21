use bevy::ecs::system::Res;
use bevy::prelude::{Transform, Vec2, KeyCode, Input, Vec3, Without, Camera2d, Camera2dBundle};
use bevy::sprite::{SpriteBundle, Sprite};
use bevy::time::Time;
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
                    custom_size: Some(Vec2::new(96.0, 96.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
            Player {},
        )
    ); 
}

pub const PLAYER_SPEED: f32 = 200.0; 

pub const MOVEMENTS_MAPPING: [(KeyCode, Vec3); 4] = [
    (KeyCode::W, Vec3::new(0.0, 1.0, 0.0)),
    (KeyCode::A, Vec3::new(-1.0, 0.0, 0.0)),
    (KeyCode::D, Vec3::new(1.0, 0.0, 0.0)),
    (KeyCode::S, Vec3::new(0.0, -1.0, 0.0))
];

fn player_query() {
    
}

pub fn player_movements(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        for (code, mov) in MOVEMENTS_MAPPING {
            if keyboard_input.pressed(code) {
                direction += mov
            }
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();

    }
}