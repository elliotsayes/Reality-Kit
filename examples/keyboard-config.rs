use std::hint;

use reality_core::game_tick::RealityGameTick;
use reality_kit::bevy::prelude::*;
use reality_kit::core::game_tick::RealityGameTickPlugin;
use reality_kit::player_interface::{
    ActionDescriptor, EventDescriptor, GameInputEvent, InputEventType, KeyboardConfig,
    PlayerInterfaceManifest, RealityInputPlugin,
};
use reality_player_interface::{GameUpdateEventTimed, PlayerInterfaceGameUpdate, UpdateInfo};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Event)]
enum MyGameEvents {
    RotationStarted,
    RotationEnded,
}

fn get_all_events() -> Vec<EventDescriptor<MyGameEvents>> {
    vec![
        EventDescriptor::new(MyGameEvents::RotationStarted)
            .desc(String::from("Cube has started rotating.")),
        EventDescriptor::new(MyGameEvents::RotationEnded)
            .desc(String::from("Cube has stopped rotating.")),
    ]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum MyGameActions {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
}

fn get_all_actions() -> Vec<ActionDescriptor<MyGameActions>> {
    vec![
        ActionDescriptor::new(MyGameActions::MoveUp).desc(String::from("Rotate the cube up")),
        ActionDescriptor::new(MyGameActions::MoveDown).desc(String::from("Rotate the cube down")),
        ActionDescriptor::new(MyGameActions::MoveLeft).desc(String::from("Rotate the cube left")),
        ActionDescriptor::new(MyGameActions::MoveRight).desc(String::from("Rotate the cube right")),
    ]
}

fn main() {
    let manifest = PlayerInterfaceManifest::<MyGameEvents, MyGameActions> {
        name: "Cube Rotator".to_string(),
        version: 1,
        tick_rate: 60,
        hint_text: Some("Rotate a cube for fun!".to_string()),
        events_global: get_all_events(),
        actions_global: get_all_actions(),
    };

    let keyboard_config =
        serde_json::from_value::<KeyboardConfig<MyGameActions>>(serde_json::json!({
            "bindings": {
                "KeyW": ["MoveUp"],
                "KeyS": ["MoveDown"],
                "KeyA": ["MoveLeft"],
                "KeyD": ["MoveRight"],
                "ArrowUp": ["MoveUp"],
                "ArrowDown": ["MoveDown"],
                "ArrowLeft": ["MoveLeft"],
                "ArrowRight": ["MoveRight"],
            }
        }))
        .unwrap();

    println!("Manifest: {manifest:#?}");
    println!("Keyboard config: {keyboard_config:#?}");

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RealityGameTickPlugin::default())
        .add_plugins(RealityInputPlugin { keyboard_config })
        .add_event::<MyGameEvents>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (set_rotation_state, rotate_camera, print_player_updates),
        )
        .run();
}

#[derive(Component)]
enum RotationState {
    None,
    Left,
    Right,
    Up,
    Down,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn((
        Camera3d { ..default() },
        Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        RotationState::None,
    ));

    // Light
    commands.spawn(PointLight {
        intensity: 1500.0,
        shadows_enabled: true,
        ..default()
    });
    commands.spawn((
        PointLight {
            intensity: 3000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // Cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.0, 2.0, 2.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.9, 0.2, 0.1))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn set_rotation_state(
    // Event added by `RealityInputPlugin`
    mut evr_gie: EventReader<GameInputEvent<MyGameActions>>,
    mut query: Query<&mut RotationState>,
    // Event added manually (for now?)
    mut evw_gue: EventWriter<MyGameEvents>,
) {
    for ev in evr_gie.read() {
        let mut rotation_state = query.single_mut();
        if ev.event_type == InputEventType::Begin {
            match ev.action {
                MyGameActions::MoveUp => *rotation_state = RotationState::Up,
                MyGameActions::MoveDown => *rotation_state = RotationState::Down,
                MyGameActions::MoveLeft => *rotation_state = RotationState::Left,
                MyGameActions::MoveRight => *rotation_state = RotationState::Right,
            }
            evw_gue.send(MyGameEvents::RotationStarted);
        } else {
            *rotation_state = RotationState::None;
            evw_gue.send(MyGameEvents::RotationEnded);
        }
    }
}

fn rotate_camera(
    mut query: Query<(&mut Transform, &RotationState), With<Camera>>,
    time: Res<Time>,
) {
    let (mut transform, rotation_state) = query.single_mut();
    let rotation_speed = 1.0;

    let rotation = match rotation_state {
        RotationState::None => Quat::IDENTITY,
        RotationState::Left => Quat::from_rotation_y(-rotation_speed * time.delta_secs()),
        RotationState::Right => Quat::from_rotation_y(rotation_speed * time.delta_secs()),
        RotationState::Up => Quat::from_rotation_x(rotation_speed * time.delta_secs()),
        RotationState::Down => Quat::from_rotation_x(-rotation_speed * time.delta_secs()),
    };

    transform.rotate_around(Vec3::ZERO, rotation);
    // Keep the camera looking at the cube
    transform.look_at(Vec3::ZERO, Vec3::Y);
}

fn print_player_updates(mut evr_gie: EventReader<MyGameEvents>, rgt: Res<RealityGameTick>) {
    for ev in evr_gie.read() {
        // println!("MyGameEvent: {ev:?}");

        let hint_text = match ev {
            MyGameEvents::RotationStarted => Some("Cube is On-screen, rotating".to_string()),
            MyGameEvents::RotationEnded => Some("Cube is On-screen, stationary".to_string()),
        };

        let update: PlayerInterfaceGameUpdate<MyGameEvents, MyGameActions> =
            PlayerInterfaceGameUpdate {
                tick_initial: rgt.tick,
                updates: vec![GameUpdateEventTimed {
                    tick_delta: 0,
                    update: UpdateInfo::GameEvent(reality_player_interface::UpdateInfo::GameEvent(
                        ev.clone(),
                    )),
                }],
                actions_current: Some(
                    get_all_actions()
                        .into_iter()
                        .map(|action_descriptor| action_descriptor.action)
                        .collect(),
                ),
                hint_text,
                state_text: None,
                media_refs: None,
            };

        // These events should be sent from the client process to subscribing agent
        println!("{update:#?}");
    }
}
