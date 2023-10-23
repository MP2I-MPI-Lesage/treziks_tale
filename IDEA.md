# Just a dump idea that doesn't work because it makes the sprites laggy

```rust
    use actions::ActionEvent;
    
    .add_systems(Update, (actions::gamepad_connections, actions::gamepad_inputs, actions::player_actions, actions::keyboard_inputs))
    .add_event::<ActionEvent>() 

```

```rust
#[derive(Resource)]
pub struct PlayerGamepad(Gamepad);

pub fn gamepad_connections(
    mut commands: Commands,
    player_gamepad: Option<Res<PlayerGamepad>>,
    mut gamepad_evr: EventReader<GamepadConnectionEvent>,
) {
    for (ev, ev_id) in gamepad_evr.iter_with_id() {
        // the ID of the gamepad
        let id = ev.gamepad;
        match &ev.connection {
            GamepadConnection::Connected(info) => {
                println!("Gamepad connected: {} [{:?}]", info.name, id);
                if player_gamepad.is_none() {
                    commands.insert_resource(PlayerGamepad(id))
                }
            },
            GamepadConnection::Disconnected => {
                println!("Gamepad disconnected: [{:?}]", id);
                // If the gamepad is the one we associated to the player: disassociate it.
                if let Some(PlayerGamepad(old)) = player_gamepad.as_deref() {
                    if *old == id {
                        commands.remove_resource::<PlayerGamepad>();
                    }
                }
            }
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
pub enum Movements {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
pub enum Actions {
    Movement(Movements),
    BaseAttack,
    SecondaryAttack,
    HeavyAttack,
    Interact,
    OpenSpellBook,
    OpenInventory,
    OpenMenu
}

#[derive(Event)]
pub struct ActionEvent(Actions);

pub const MOVEMENTS_MAPPING: [(Movements, Vec3); 4] = [
    (Movements::North, Vec3::new(0.0, 1.0, 0.0)),
    (Movements::West, Vec3::new(-1.0, 0.0, 0.0)),
    (Movements::East, Vec3::new(1.0, 0.0, 0.0)),
    (Movements::South, Vec3::new(0.0, -1.0, 0.0))
];

// Will send an ActionEvent
pub fn gamepad_inputs(
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<Input<GamepadButton>>,
    player_gamepad: Option<Res<PlayerGamepad>>,
    mut ev_action: EventWriter<ActionEvent>
) {
    let gamepad = if let Some(gp) = player_gamepad {
        gp.0
    } else {
        return;
    };

    // The joysticks are represented using a separate axis for X and Y
    let axis_lx = GamepadAxis {
        gamepad,
        axis_type: GamepadAxisType::LeftStickX
    };

    let axis_ly = GamepadAxis {
        gamepad,
        axis_type: GamepadAxisType::LeftStickY
    };

    if let (Some(x), Some(y)) = (axes.get(axis_lx), axes.get(axis_ly)) {
        // combine X and Y into one vector
        let left_stick_pos = Vec2::new(x, y);

        // Stick is pushed up
        if left_stick_pos.length() > 0.9 && left_stick_pos.y > 0.5 {
            ev_action.send(ActionEvent(Actions::Movement(Movements::North)))
        }

        // Stick is pushed down
        if left_stick_pos.length() > 0.9 && left_stick_pos.y < 0.5 {
            ev_action.send(ActionEvent(Actions::Movement(Movements::South)))
        }

        // Stick is pushed right
        if left_stick_pos.length() > 0.9 && left_stick_pos.x > 0.5 {
            ev_action.send(ActionEvent(Actions::Movement(Movements::East)))
        }
        // Stick is pushed left
        if left_stick_pos.length() > 0.9 && left_stick_pos.x < 0.5 {
            ev_action.send(ActionEvent(Actions::Movement(Movements::West)))
        }

    }

}

pub fn keyboard_inputs(
    keyboard_input: Res<Input<KeyCode>>,
    mut ev_action: EventWriter<ActionEvent>,
) {
    let map = [
        (KeyCode::W, Movements::North),
        (KeyCode::A, Movements::West),
        (KeyCode::S, Movements::South),
        (KeyCode::D, Movements::East)
    ];

    for (code, mov) in map {
        if keyboard_input.pressed(code) {
            ev_action.send(ActionEvent(Actions::Movement(mov)));
        }
    } 
}

pub fn player_actions(
    mut ev_action: EventReader<ActionEvent>,
    mut q: Query<&mut Transform, With<Player>>,
    time: Res<Time>
) {
    let mut direction = Vec3::ZERO;
    for e in ev_action.iter() {
        match &e.0 {
            Actions::Movement(m) => {
                if let Ok(mut transform) = q.get_single_mut() {
                    for (code, mov) in MOVEMENTS_MAPPING {
                        if m == &code {
                            direction += mov
                        }
                    }
                    if direction.length() > 0.0 {
                        direction = direction.normalize();
                    }
                    transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
                } 
            }
            a => println!("Issued action: {:?}", a)
        }
    }
}
```