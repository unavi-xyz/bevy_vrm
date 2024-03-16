use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_egui::{egui::Window, EguiContexts, EguiPlugin};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_shader_mtoon::MtoonSun;
use bevy_vrm::{BoneName, HumanoidBones, SpringBones, VrmBundle, VrmPlugin};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .init_resource::<Settings>()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                file_path: "../../assets".to_string(),
                ..default()
            }),
            EguiPlugin,
            PanOrbitCameraPlugin,
            VrmPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (draw_spring_bones, move_leg, ui))
        .run();
}

const MODELS: [&str; 3] = ["catbot.vrm", "cool_loops.vrm", "suzuha.vrm"];
const PATH: &str = MODELS[2];

fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut config: ResMut<GizmoConfigStore>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let (config, _) = config.config_mut::<DefaultGizmoConfigGroup>();
    config.depth_bias = -1.0;

    commands.spawn(VrmBundle {
        vrm: asset_server.load(PATH.to_string()),
        ..default()
    });

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(1.0, 2.0, -3.0),
            ..Default::default()
        },
        PanOrbitCamera {
            focus: Vec3::new(0.0, 1.0, 0.0),
            ..Default::default()
        },
    ));

    let mut sun_transform = Transform::default();
    sun_transform.rotate_x(-PI / 1.3);
    sun_transform.rotate_y(PI / 6.0);

    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 10_000.0,
                shadows_enabled: true,
                ..default()
            },
            transform: sun_transform,
            ..default()
        },
        MtoonSun,
    ));

    commands.spawn(PbrBundle {
        material: materials.add(StandardMaterial::default()),
        mesh: meshes.add(Plane3d::default()),
        transform: Transform::from_scale(Vec3::splat(30.0)),
        ..default()
    });
}

#[derive(Default, Resource)]
struct Settings {
    show_gizmos: bool,
    move_leg: bool,
}

fn move_leg(
    mut original_transform: Local<Option<Transform>>,
    mut transforms: Query<&mut Transform>,
    settings: Res<Settings>,
    time: Res<Time>,
    vrm: Query<&HumanoidBones>,
) {
    for humanoid in vrm.iter() {
        let leg = match humanoid.0.get(&BoneName::RightUpperLeg) {
            Some(leg) => leg,
            None => continue,
        };

        if !settings.move_leg {
            if let Some(original_transform) = original_transform.as_ref() {
                if let Ok(mut transform) = transforms.get_mut(*leg) {
                    *transform = *original_transform;
                }
            }
            return;
        }

        if let Ok(mut transform) = transforms.get_mut(*leg) {
            if original_transform.is_none() {
                *original_transform = Some(*transform);
            }

            let sin = time.elapsed_seconds().sin();
            transform.rotation = Quat::from_rotation_x(sin);
        }
    }
}

fn draw_spring_bones(
    mut gizmos: Gizmos,
    spring_bones: Query<&SpringBones>,
    transforms: Query<&GlobalTransform>,
    settings: Res<Settings>,
) {
    if !settings.show_gizmos {
        return;
    }

    for spring_bones in spring_bones.iter() {
        for spring_bone in spring_bones.0.iter() {
            for bone_entity in spring_bone.bones.iter() {
                let position = transforms.get(*bone_entity).unwrap().translation();

                gizmos.sphere(
                    position,
                    Quat::default(),
                    spring_bone.hit_radius,
                    Color::rgb(spring_bone.stiffness, 1.0 - spring_bone.stiffness, 0.1),
                );
            }
        }
    }
}

fn ui(mut contexts: EguiContexts, mut settings: ResMut<Settings>) {
    Window::new("bevy_vrm").show(contexts.ctx_mut(), |ui| {
        ui.checkbox(&mut settings.show_gizmos, "Show spring bones");
        ui.checkbox(&mut settings.move_leg, "Move leg bone");
    });
}
