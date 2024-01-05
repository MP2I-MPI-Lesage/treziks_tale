use bevy::{prelude::Commands, ui::{node_bundles::{NodeBundle, ImageBundle}, Style, Val, JustifyContent, UiImage}, ecs::{system::{Query, Res}, query::With, component::Component, entity::Entity}, asset::AssetServer, hierarchy::BuildChildren, render::texture::Image};

#[derive(Component)]
pub struct Health(pub i8);

pub const MAX_HEALTH: i8 = 9;
pub const HEART_COUNT: i8 = 3;

impl Health {
    pub fn to_hearts(&self) -> (i8, i8) {
        let hearts = self.0 / HEART_COUNT;
        let heart_pieces = self.0 % HEART_COUNT;
        (hearts, heart_pieces)
    }
}