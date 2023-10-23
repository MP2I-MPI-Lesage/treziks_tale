
use bevy::{prelude::{Component, Commands, Query, With, Camera2dBundle, Transform, Without, OrthographicProjection}, window::{Window, PrimaryWindow}, render::camera::ScalingMode};

use crate::player::Player;



#[derive(Component)]
pub struct PlayerCamera;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap(); 

    let scale = 0.4;

    let mut camera = Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0), 
        ..Default::default()
    };

    camera.projection.scale = scale;
    camera.projection.far = 1000.0 / scale;

    commands.spawn(
        (
            camera,       
            PlayerCamera,
        )
    );
}

pub fn followup_camera(p_query: Query<&Transform, (With<Player>, Without<PlayerCamera>)>, mut c_query: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>) { 
    let p_transform = p_query.get_single().unwrap();
    let mut _c_transform = c_query.get_single_mut().unwrap(); 
    
    _c_transform.translation = p_transform.translation
}