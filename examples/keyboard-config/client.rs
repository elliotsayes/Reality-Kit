use reality_kit::bevy::prelude::*;
use reality_player_interface::{
    ActionType, GameActionEvent, KeyboardConfig, PlayerInterfaceManifest, RealityInputPlugin,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
enum MyGameActions {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
}

const ALL_ACTIONS: [MyGameActions; 4] = [
    MyGameActions::MoveUp,
    MyGameActions::MoveDown,
    MyGameActions::MoveLeft,
    MyGameActions::MoveRight,
];

fn main() {
    let manifest = PlayerInterfaceManifest::<MyGameActions> {
        name: "Test".to_string(),
        version: 1,
        tick_rate: 60,
        actions_global: ALL_ACTIONS.to_vec(),
        actions_current: None,
    };

    let keyboard_config =
        serde_json::from_value::<KeyboardConfig<MyGameActions>>(serde_json::json!({
            "bindings": {
                "KeyW": ["MoveUp"],
                "KeyS": ["MoveDown"],
                "KeyA": ["MoveLeft"],
                "KeyD": ["MoveRight"],
            }
        }))
        .unwrap();

    println!("Manifest: {manifest:#?}");
    println!("Keyboard config: {keyboard_config:#?}");

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RealityInputPlugin { keyboard_config })
        .add_systems(Startup, setup)
        .add_systems(Update, (set_rotation_state, rotate_camera))
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
        Camera3dBundle {
            transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        RotationState::None,
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
    mut evr_gae: EventReader<GameActionEvent<MyGameActions>>,
    mut query: Query<&mut RotationState>,
) {
    for ev in evr_gae.read() {
        let mut rotation_state = query.single_mut();
        if ev.event == ActionType::Begin {
            match ev.action {
                MyGameActions::MoveUp => *rotation_state = RotationState::Up,
                MyGameActions::MoveDown => *rotation_state = RotationState::Down,
                MyGameActions::MoveLeft => *rotation_state = RotationState::Left,
                MyGameActions::MoveRight => *rotation_state = RotationState::Right,
            }
        } else {
            *rotation_state = RotationState::None;
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
        RotationState::Left => Quat::from_rotation_y(-rotation_speed * time.delta_seconds()),
        RotationState::Right => Quat::from_rotation_y(rotation_speed * time.delta_seconds()),
        RotationState::Up => Quat::from_rotation_x(rotation_speed * time.delta_seconds()),
        RotationState::Down => Quat::from_rotation_x(-rotation_speed * time.delta_seconds()),
    };

    transform.rotate_around(Vec3::ZERO, rotation);
    // Keep the camera looking at the cube
    transform.look_at(Vec3::ZERO, Vec3::Y);
}
