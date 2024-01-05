use bevy::{ecs::{system::{Commands, Query, ResMut}, query::{Changed, With}, schedule::NextState}, ui::{BackgroundColor, Interaction}, render::color::Color};

use crate::AppState;

use super::components::EnterGameButton;

pub fn enter_game_interaction(mut query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<EnterGameButton>)>, mut app_next_state: ResMut<NextState<AppState>>) {
    if let Ok((interaction, mut bg_color)) = query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => app_next_state.set(AppState::InGame),
            Interaction::Hovered => *bg_color = BackgroundColor(Color::BLUE),
            _ => ()
        }
    }
}