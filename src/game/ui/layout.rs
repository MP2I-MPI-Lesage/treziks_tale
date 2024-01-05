use bevy::{ecs::{system::{Commands, Res, Query}, query::With, entity::Entity}, asset::AssetServer, ui::{node_bundles::{NodeBundle, ImageBundle}, Style, Val, JustifyContent, UiImage, FlexDirection}, hierarchy::BuildChildren};

use crate::game::player::{Player, health::{Health, MAX_HEALTH, HEART_COUNT}};

use super::components::{HealthBar, heart_entity, HeartType, int_to_heart_pieces};

pub fn spawn_health_bar(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(100.0),
                justify_content: JustifyContent::FlexStart,
                ..Default::default()
            },
            ..Default::default()
        }
    )
    .with_children(|parent| {
        parent.spawn(
            (
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                HealthBar {},
            )
        );
    });
}

pub fn update_health_bar(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut health_bar_query: Query<Entity, With<HealthBar>>,
    health_query: Query<&Health, With<Player>> 
) {
    if let Ok(health) = health_query.get_single() {
        let (hearts, hearts_pieces) = health.to_hearts();
        let mut health_bar_entity = health_bar_query.get_single_mut().unwrap();
        let mut health_bar = commands.get_entity(health_bar_entity).unwrap();

        // Clear all hearts from the bar
        health_bar.clear_children();
        
        for _ in 1..hearts {
            health_bar.add_child(heart_entity(commands, HeartType::Full, asset_server));
        }

        let mut pieces = false;

        if hearts_pieces > 0 {
            health_bar.add_child(heart_entity(commands, int_to_heart_pieces(hearts_pieces), asset_server));
            pieces = true;
        };

        // Determine the number of empty hearts to display
        if HEART_COUNT - hearts > 0 {
            if pieces {
                for _ in 1..(HEART_COUNT - hearts - 1) {
                    health_bar.add_child(heart_entity(commands, HeartType::Empty, asset_server));
                }
            } else {
                for _ in 1..(HEART_COUNT - hearts) {
                    health_bar.add_child(heart_entity(commands, HeartType::Empty, asset_server));
                }
            }
        }
    };
}