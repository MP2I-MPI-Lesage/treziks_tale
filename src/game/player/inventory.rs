use bevy::prelude::*;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_inventory);
    }
}

pub fn spawn_inventory(mut commands: Commands) {
    commands.spawn(NodeBundle {
        ..Default::default()
    });
}