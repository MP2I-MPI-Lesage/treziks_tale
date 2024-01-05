use bevy::{ecs::{component::Component, entity::Entity, system::{Res, Commands}}, ui::{node_bundles::ImageBundle, UiImage}, asset::AssetServer};


#[derive(Component)]
pub struct HealthBar {}

pub enum HeartType {
    Full,
    ThreeQuarters,
    Half,
    Quarter,
    Empty
}

pub fn int_to_heart_pieces(i: i8) -> HeartType {
    match i {
        1 => HeartType::Quarter,
        2 => HeartType::Half,
        3 => HeartType::ThreeQuarters,
        _ => panic!("There is more heart than expected.")
    }
}

pub fn heart_entity(mut commands: Commands, style: HeartType, asset_server: Res<AssetServer>) -> Entity {
    let filename = match style {
        HeartType::Full => "full",
        HeartType::Half => "half",
        HeartType::Quarter => "quarter",
        HeartType::ThreeQuarters => "threequarters",
        HeartType::Empty => "empty"
    }; 
    
    commands.spawn(
        ImageBundle {
            image: UiImage {
                texture: asset_server.load(format!("sprites/ui/health/{}.png", filename)),
                ..Default::default()
            },
            ..Default::default()
        }
    ).id()
}