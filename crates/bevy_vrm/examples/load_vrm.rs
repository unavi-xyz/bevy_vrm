use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_shader_mtoon::{MtoonMainCamera, MtoonSun};
use bevy_vrm::{BoneName, HumanoidBones, SpringBones, VrmBundle, VrmPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                file_path: "../../assets".to_string(),
                ..default()
            }),
            PanOrbitCameraPlugin,
            VrmPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (draw_spring_bones, move_leg))
        .run();
}

const MODELS: [&str; 3] = ["catbot.vrm", "cool_loops.vrm", "suzuha.vrm"];
const PATH: &str = MODELS[2];

fn setup(
    mut commands: Commands,
    mut config: ResMut<GizmoConfigStore>,
    asset_server: Res<AssetServer>,
) {
    let (config, _) = config.config_mut::<DefaultGizmoConfigGroup>();
    config.depth_bias = -1.0;

    commands.spawn(VrmBundle {
        vrm: asset_server.load(PATH.to_string()),
        ..default()
    });

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(2.0, 2.0, -3.0),
            ..Default::default()
        },
        MtoonMainCamera,
        PanOrbitCamera {
            focus: Vec3::new(0.0, 1.0, 0.0),
            ..Default::default()
        },
    ));

    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 5000.0,
                ..default()
            },
            transform: Transform::from_xyz(2.0, 8.0, 5.0),
            ..default()
        },
        MtoonSun,
    ));
}

fn move_leg(mut transforms: Query<&mut Transform>, time: Res<Time>, vrm: Query<&HumanoidBones>) {
    for humanoid in vrm.iter() {
        let leg = match humanoid.0.get(&BoneName::RightUpperLeg) {
            Some(leg) => leg,
            None => continue,
        };

        if let Ok(mut transform) = transforms.get_mut(*leg) {
            let sin = time.elapsed_seconds().sin();
            transform.rotation = Quat::from_rotation_x(sin);
        }
    }
}

fn draw_spring_bones(
    mut gizmos: Gizmos,
    spring_bones: Query<&SpringBones>,
    transforms: Query<&GlobalTransform>,
) {
    for spring_bones in spring_bones.iter() {
        for spring_bone in spring_bones.0.iter() {
            for bone_entity in spring_bone.bones.iter() {
                let position = transforms.get(*bone_entity).unwrap().translation();
                gizmos.sphere(
                    position,
                    Quat::default(),
                    spring_bone.hit_radius,
                    Color::rgb(10.0 / spring_bone.stiffiness, 0.2, 0.2),
                );
            }
        }
    }
}
