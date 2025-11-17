use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_vrm::mtoon::MtoonSun;
use bevy_vrm::{VrmInstance, VrmPlugins};

#[derive(Component)]
struct LinearMotion {
    amplitude: f32,
    offset: f32,
    speed: f32,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                file_path: "../../assets".to_string(),
                ..default()
            }),
            VrmPlugins,
            PanOrbitCameraPlugin,
        ))
        .add_systems(Startup, setup_scene)
        .add_systems(Update, animate_linear_motion)
        .run();
}

fn setup_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera.
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.0, 10.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
        PanOrbitCamera {
            focus: Vec3::new(0.0, 1.0, 0.0),
            ..default()
        },
    ));

    // Light.
    commands.spawn((
        DirectionalLight {
            illuminance: 10_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_rotation_x(-PI / 3.0)),
        MtoonSun,
    ));

    // Avatar 1: Medium speed moving back and forth.
    commands.spawn((
        Transform::from_xyz(-3.0, 0.0, 0.0),
        VrmInstance(asset_server.load("suzuha.vrm")),
        LinearMotion {
            amplitude: 3.0,
            speed: 1.0,
            offset: 0.0,
        },
    ));

    // Avatar 2: Slower movement.
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        VrmInstance(asset_server.load("suzuha.vrm")),
        LinearMotion {
            amplitude: 4.0,
            speed: 0.7,
            offset: PI / 2.0,
        },
    ));

    // Avatar 3: Faster movement with phase offset.
    commands.spawn((
        Transform::from_xyz(3.0, 0.0, 0.0),
        VrmInstance(asset_server.load("suzuha.vrm")),
        LinearMotion {
            amplitude: 2.5,
            speed: 1.5,
            offset: PI,
        },
    ));
}

fn animate_linear_motion(time: Res<Time>, mut query: Query<(&mut Transform, &LinearMotion)>) {
    for (mut transform, motion) in query.iter_mut() {
        let t = time.elapsed_secs() * motion.speed + motion.offset;
        let z = t.sin() * motion.amplitude;
        transform.translation.z = z;

        // Rotate to face movement direction.
        let rotation = if z > 0.0 { 0.0 } else { PI };
        transform.rotation = Quat::from_rotation_y(rotation);
    }
}
