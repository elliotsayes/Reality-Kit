use std::ops::Deref;

use reality_kit::bevy::prelude::*;
use reality_player_interface::{GameActionEvent, KeyboardConfig, RealityInputPlugin};

fn main() {
    let keyboard_config: KeyboardConfig = serde_json::from_value(serde_json::json!({
        "bindings": {
            "KeyW": ["MoveUp"],
            "KeyS": ["MoveDown"],
            "KeyA": ["MoveLeft"],
            "KeyD": ["MoveRight"],
        }
    }))
    .unwrap();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RealityInputPlugin { keyboard_config })
        .add_systems(Startup, setup)
        .add_systems(Update, (set_rotation_state, rotate_camera))
        .run();
}

#[derive(Component)]
enum RotationState {
    ClockwiseLeft,
    ClockwiseRight,
    CounterClockwiseLeft,
    CounterClockwiseRight,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        RotationState::ClockwiseLeft,
    ));

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // Cube
    let cube = Cuboid::new(1.0, 1.0, 1.0);
    commands.spawn(PbrBundle {
        mesh: meshes.add(cube),
        material: materials.add(Color::srgb(0.8, 0.7, 0.6)),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}

fn set_rotation_state(
    mut evr_gae: EventReader<GameActionEvent>,
    mut query: Query<&mut RotationState>,
) {
    for ev in evr_gae.read() {
        let mut rotation_state = query.single_mut();
        match ev.action.deref() {
            "MoveUp" => *rotation_state = RotationState::ClockwiseLeft,
            "MoveDown" => *rotation_state = RotationState::CounterClockwiseLeft,
            "MoveLeft" => *rotation_state = RotationState::CounterClockwiseRight,
            "MoveRight" => *rotation_state = RotationState::ClockwiseRight,
            _ => {}
        }
    }
}

fn rotate_camera(
    mut query: Query<(&mut Transform, &RotationState), With<Camera>>,
    time: Res<Time>,
) {
    let (mut transform, rotation_state) = query.single_mut();
    let rotation_speed = 0.5;

    match rotation_state {
        RotationState::ClockwiseLeft => {
            let rotation = Quat::from_rotation_y(time.delta_seconds() * rotation_speed);
            transform.rotate_around(Vec3::ZERO, rotation);
        }
        RotationState::ClockwiseRight => {
            let rotation = Quat::from_rotation_y(-time.delta_seconds() * rotation_speed);
            transform.rotate_around(Vec3::ZERO, rotation);
        }
        RotationState::CounterClockwiseLeft => {
            let rotation = Quat::from_rotation_x(-time.delta_seconds() * rotation_speed);
            transform.rotate_around(Vec3::ZERO, rotation);
        }
        RotationState::CounterClockwiseRight => {
            let rotation = Quat::from_rotation_x(time.delta_seconds() * rotation_speed);
            transform.rotate_around(Vec3::ZERO, rotation);
        }
    }

    // Keep the camera looking at the cube
    transform.look_at(Vec3::ZERO, Vec3::Y);
}
