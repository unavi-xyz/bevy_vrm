use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_vrm::first_person::{FirstPersonFlag, SetupFirstPerson};
use bevy_vrm::loader::Vrm;
use bevy_vrm::mtoon::MtoonSun;
use bevy_vrm::{VrmInstance, VrmPlugins};

#[derive(Component)]
struct LinearMotion {
    amplitude: f32,
    offset: f32,
    speed: f32,
}

/// Marker component to override first person flag for this avatar.
#[derive(Component)]
struct OverrideFirstPersonFlag(FirstPersonFlag);

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
        .add_systems(
            Update,
            (
                animate_linear_motion,
                setup_first_person,
                override_first_person_flags,
            ),
        )
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

    // Avatar 1: Medium speed, Auto first person flag (default).
    commands.spawn((
        Transform::from_xyz(-3.0, 0.0, 0.0),
        VrmInstance(asset_server.load("alicia.vrm")),
        LinearMotion {
            amplitude: 3.0,
            speed: 1.0,
            offset: 0.0,
        },
        OverrideFirstPersonFlag(FirstPersonFlag::Auto),
    ));

    // Avatar 2: Slower movement, ThirdPersonOnly flag.
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        VrmInstance(asset_server.load("alicia.vrm")),
        LinearMotion {
            amplitude: 4.0,
            speed: 0.7,
            offset: PI / 2.0,
        },
        OverrideFirstPersonFlag(FirstPersonFlag::ThirdPersonOnly),
    ));

    // Avatar 3: Faster movement, Both flag.
    commands.spawn((
        Transform::from_xyz(3.0, 0.0, 0.0),
        VrmInstance(asset_server.load("alicia.vrm")),
        LinearMotion {
            amplitude: 2.5,
            speed: 1.5,
            offset: PI,
        },
        OverrideFirstPersonFlag(FirstPersonFlag::Both),
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

fn setup_first_person(
    mut events: MessageReader<AssetEvent<Vrm>>,
    mut writer: MessageWriter<SetupFirstPerson>,
    vrms: Query<(Entity, &VrmInstance)>,
) {
    for event in events.read() {
        if let AssetEvent::LoadedWithDependencies { id } = event {
            let Some((entity, _)) = vrms.iter().find(|(_, handle)| handle.0.id() == *id) else {
                continue;
            };

            writer.write(SetupFirstPerson(entity));
        }
    }
}

fn override_first_person_flags(
    avatars: Query<(Entity, &OverrideFirstPersonFlag), With<VrmInstance>>,
    mut flags: Query<&mut FirstPersonFlag>,
    children: Query<&Children>,
) {
    for (avatar_entity, override_flag) in avatars.iter() {
        // Recursively traverse all descendants to find and update FirstPersonFlag components.
        let mut stack = vec![avatar_entity];
        while let Some(entity) = stack.pop() {
            if let Ok(mut flag) = flags.get_mut(entity) {
                *flag = override_flag.0;
            }
            if let Ok(children) = children.get(entity) {
                stack.extend(children.iter());
            }
        }
    }
}
