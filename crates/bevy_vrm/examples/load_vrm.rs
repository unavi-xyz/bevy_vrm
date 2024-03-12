use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_shader_mtoon::{MtoonMainCamera, MtoonSun};
use bevy_vrm::{VrmBundle, VrmPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                file_path: "../../assets".to_string(),
                ..default()
            }),
            VrmPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_vrm)
        .run();
}

const MODELS: [&str; 3] = ["catbot.vrm", "cool_loops.vrm", "suzuha.vrm"];
const PATH: &str = MODELS[2];

#[derive(Component)]
struct VrmTag;

fn setup(
    mut commands: Commands,
    mut config: ResMut<GizmoConfigStore>,
    asset_server: Res<AssetServer>,
) {
    let (config, _) = config.config_mut::<DefaultGizmoConfigGroup>();
    config.depth_bias = -1.0;

    let mut transform = Transform::from_xyz(0.0, -1.0, -4.0);
    transform.rotate_y(PI);

    commands.spawn((
        VrmBundle {
            vrm: asset_server.load(PATH),
            scene_bundle: SceneBundle {
                transform,
                ..default()
            },
        },
        VrmTag,
    ));

    commands.spawn((Camera3dBundle::default(), MtoonMainCamera));

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
        MtoonSun,
    ));
}
fn rotate_vrm(time: Res<Time>, mut query: Query<&mut Transform, With<VrmTag>>) {
    for mut transform in query.iter_mut() {
        transform.rotate(Quat::from_rotation_y(time.delta_seconds() / 3.0));
    }
}

// fn move_arm(
//     time: Res<Time>,
//     mut transforms: Query<&mut Transform, Without<bevy_vrm::HumanoidBones>>,
//     humanoid_bones: Query<&bevy_vrm::HumanoidBones>,
// ) {
//     for humanoid_bones in humanoid_bones.iter() {
//         transforms
//             .get_mut(humanoid_bones.left_hand)
//             .unwrap()
//             .rotate(Quat::from_rotation_x(time.delta_seconds() * 1.5));
//         transforms
//             .get_mut(humanoid_bones.right_lower_leg)
//             .unwrap()
//             .rotate(Quat::from_rotation_z(time.delta_seconds() * 1.5));
//     }
// }
//
// fn draw_spring_bones(
//     mut gizmos: Gizmos,
//     spring_bones: Query<&SpringBones>,
//     transforms: Query<&GlobalTransform>,
// ) {
//     for spring_bones in spring_bones.iter() {
//         for spring_bone in spring_bones.0.iter() {
//             for bone_entity in spring_bone.bones.iter() {
//                 let position = transforms.get(*bone_entity).unwrap().translation();
//                 gizmos.sphere(
//                     position,
//                     Quat::default(),
//                     spring_bone.hit_radius,
//                     Color::rgb(10.0 / spring_bone.stiffiness, 0.2, 0.2),
//                 );
//             }
//         }
//     }
// }
