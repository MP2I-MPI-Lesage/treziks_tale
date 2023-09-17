mod player;

use bevy::{prelude::{App, Startup}, DefaultPlugins};


fn main() {
    App::new().add_plugins(DefaultPlugins)
    .add_systems(Startup, player::setup).run()


}