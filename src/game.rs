pub mod player;
pub mod ui;
use bevy::app::{Plugin, App};

use self::player::PlayerPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(PlayerPlugin);
    }
}