use bevy::prelude::*;
use std::f32::consts::PI;

use bevy_vrm::VrmPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, VrmPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_vrm)
        .run();
}

const MODELS: [&str; 3] = ["catbot.vrm", "cool_loops.vrm", "suzuha.vrm"];
const MODEL_INDEX: usize = 2;

#[derive(Component)]
struct VrmTag;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut transform = Transform::from_xyz(0.0, -1.0, -4.0);
    transform.rotate_y(PI);

    commands.spawn((
        SceneBundle {
            scene: asset_server.load(format!("{}#Scene0", MODELS[MODEL_INDEX]).as_str()),
            transform,
            ..default()
        },
        VrmTag,
    ));

    commands.spawn((Camera3dBundle::default(), bevy_vrm::mtoon::MtoonMainCamera));

    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: true,
                illuminance: 10_000.0,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 5.0, -5.0),
            ..default()
        },
        bevy_vrm::mtoon::MtoonSun,
    ));
}

fn rotate_vrm(time: Res<Time>, mut query: Query<&mut Transform, With<VrmTag>>) {
    for mut transform in query.iter_mut() {
        transform.rotate(Quat::from_rotation_y(time.delta_seconds() / 3.0));
    }
}
