use bevy::prelude::*;
use std::f32::consts::PI;

use bevy_vrm::VRMPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, VRMPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_vrm)
        .run();
}

#[derive(Component)]
struct VRMTag;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut transform = Transform::from_xyz(0.0, -1.0, -4.0);
    transform.rotate_y(PI);

    commands.spawn((
        SceneBundle {
            scene: asset_server.load("suzuha.vrm#Scene0"),
            transform,
            ..default()
        },
        VRMTag,
    ));

    commands.spawn(Camera3dBundle::default());
}

fn rotate_vrm(time: Res<Time>, mut query: Query<&mut Transform, With<VRMTag>>) {
    for mut transform in query.iter_mut() {
        transform.rotate(Quat::from_rotation_y(time.delta_seconds()));
    }
}
