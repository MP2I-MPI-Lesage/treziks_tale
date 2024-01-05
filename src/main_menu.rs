use bevy::{app::{App, Plugin, Startup, Update}, ecs::schedule::{OnEnter, OnExit, IntoSystemConfigs, common_conditions::in_state}};

use crate::AppState;


mod components;
mod layout;
mod styles;
mod systems;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::MainMenu), layout::spawn_main_menu)
            .add_systems(OnExit(AppState::MainMenu), layout::destroy_main_menu)
            .add_systems(Update, systems::enter_game_interaction.run_if(in_state(AppState::MainMenu)));

    }
}