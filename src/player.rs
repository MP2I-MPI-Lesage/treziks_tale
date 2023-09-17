use bevy::{prelude::{Component, With}, ecs::system::Command, window::{Window, PrimaryWindow}};

#[derive(Component)]
pub struct Player;

pub fn setup() {
    println!("The player system still works")
}

pub fn spawn_player(mut command: Command, query: Query<&Window, With<PrimaryWindow>>) {

}
