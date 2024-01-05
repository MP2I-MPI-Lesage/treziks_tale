use bevy::{prelude::{Res, Vec3, Query, With, Transform}, time::Time, reflect::Reflect};
 
use leafwing_input_manager::{Actionlike, prelude::ActionState, orientation::Direction};

use crate::game::player::Player;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerActions {
    // Movs
    Up,
    Down,
    Left,
    Right,
    // Abilities
    Attack,
    SecondaryAttack,
    HeavyAttack,
}

impl PlayerActions {
    // Lists like this can be very useful for quickly matching subsets of actions
    const DIRECTIONS: [Self; 4] = [
        PlayerActions::Up,
        PlayerActions::Down,
        PlayerActions::Left,
        PlayerActions::Right,
    ];

    fn direction(self) -> Option<Direction> {
        match self {
            PlayerActions::Up => Some(Direction::NORTH),
            PlayerActions::Down => Some(Direction::SOUTH),
            PlayerActions::Left => Some(Direction::WEST),
            PlayerActions::Right => Some(Direction::EAST),
            _ => None,
        }
    }
}

pub const PLAYER_SPEED: f32 = 200.0;

pub fn player_movements(
    mut q: Query<(&ActionState<PlayerActions>, &mut Transform), With<Player>>,
    time: Res<Time>
) {
    let (state, mut transform) = q.get_single_mut().unwrap();
    let mut direction_vec = Vec3::ZERO;
    
    for input_direction in PlayerActions::DIRECTIONS {
        if state.pressed(input_direction) {
            if let Some(direction) = input_direction.direction() {
                // Sum the directions as 2D vectors
                direction_vec += Vec3::from(direction);
            }
        }
    }

    if direction_vec.length() > 0.0 {
        direction_vec = direction_vec.normalize();
    }

    transform.translation += direction_vec * PLAYER_SPEED * time.delta_seconds();
}

