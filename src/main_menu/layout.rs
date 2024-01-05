use std::fmt::Alignment;

use bevy::{ecs::{system::{Commands, Res, Query}, query::With, entity::Entity}, asset::AssetServer, ui::{node_bundles::{NodeBundle, TextBundle, ButtonBundle}, Style, JustifyContent, AlignItems, UiRect, Val, FlexDirection, widget::Button, BorderColor}, hierarchy::{BuildChildren, DespawnRecursiveExt}, text::{Text, TextSection, TextAlignment}, render::{color::Color, render_resource::RawComputePipelineDescriptor}};

use super::components::{EnterGameButton, MainMenu};


pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(
        (
            NodeBundle  {
                style: Style {
                    height: Val::Percent(100.0),
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                ..Default::default()
            },
            MainMenu {}
        )
    ).with_children(|parent| {
        parent.spawn(
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::FlexStart,
                    margin: UiRect {
                        left: Val::Vw(10.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            }
        ).with_children(|parent| {
            parent.spawn(
                TextBundle {
                    text: Text {
                        sections: vec![TextSection::new("Trezik's Tale", bevy::text::TextStyle { font: asset_server.load("fonts/FiraCode.ttf"), font_size: 24.0, color: Color::RED })],
                        alignment: TextAlignment::Left,
                        ..Default::default()
                    },
                    ..Default::default()
                }
            );
            parent.spawn((
                ButtonBundle {
                    border_color: BorderColor(Color::WHITE),
                    style: Style {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect {
                            top: Val::Px(10.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                },
                EnterGameButton {},
            )).with_children(|parent| {
                parent.spawn(
                    TextBundle {
                        style: Style {
                            margin: UiRect {
                                left: Val::Px(10.0),
                                right: Val::Px(10.0),
                                top: Val::Px(10.0),
                                bottom: Val::Px(10.0),
                            },
                            ..Default::default()
                        },
                        text: Text {
                            sections: vec![TextSection::new("Enter Game", bevy::text::TextStyle { font: asset_server.load("fonts/FiraCode.ttf"), font_size: 24.0, color: Color::RED })],
                            alignment: TextAlignment::Left,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                );
            });
        });
    });
}

pub fn destroy_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}