//! Drag and drop [VRM](https://vrm.dev/) viewer using [bevy_vrm](https://github.com/unavi-xyz/bevy_vrm).

use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_vrm::{loader::Vrm, mtoon::MtoonSun, VrmBundle, VrmPlugins};

mod draw_spring_bones;
mod move_leg;
mod ui;

pub struct VrmViewerPlugin;

impl Plugin for VrmViewerPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(target_family = "wasm")]
        {
            app.add_plugins(bevy_web_file_drop::WebFileDropPlugin);
        }

        app.insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
            .init_resource::<Settings>()
            .add_plugins((DefaultPlugins, EguiPlugin, PanOrbitCameraPlugin, VrmPlugins))
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    draw_spring_bones::draw_spring_bones,
                    move_leg::move_leg,
                    read_dropped_files,
                    ui::update_ui,
                ),
            );
    }
}

#[derive(Resource, Default)]
struct Settings {
    pub draw_spring_bones: bool,
    pub move_leg: bool,
}

#[cfg(target_family = "wasm")]
const VRM_PATH: &str = "/bevy_vrm/assets/suzuha.vrm";
#[cfg(not(target_family = "wasm"))]
const VRM_PATH: &str = "suzuha.vrm";

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(1.0, 2.0, 5.0),
            ..default()
        },
        PanOrbitCamera {
            focus: Vec3::new(0.0, 0.8, 0.0),
            ..default()
        },
    ));

    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 10_000.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_rotation(Quat::from_rotation_x(-PI / 3.0)),
            ..default()
        },
        MtoonSun,
    ));

    let mut transform = Transform::default();
    transform.rotate_y(PI);

    commands.spawn(VrmBundle {
        scene_bundle: SceneBundle {
            transform,
            ..default()
        },
        vrm: asset_server.load(VRM_PATH),
        ..default()
    });
}

fn read_dropped_files(
    mut commands: Commands,
    mut events: EventReader<FileDragAndDrop>,
    asset_server: Res<AssetServer>,
    mut vrms: Query<Entity, With<Handle<Vrm>>>,
) {
    for event in events.read() {
        if let FileDragAndDrop::DroppedFile { path_buf, .. } = event {
            #[cfg(target_family = "wasm")]
            let path = String::from(path_buf.to_str().unwrap());
            #[cfg(not(target_family = "wasm"))]
            let path = bevy::asset::AssetPath::from_path(path_buf.as_path());

            info!("DroppedFile: {}", path);

            let entity = vrms.single_mut();
            commands.entity(entity).despawn_recursive();

            let mut transform = Transform::default();
            transform.rotate_y(PI);

            commands.spawn(VrmBundle {
                scene_bundle: SceneBundle {
                    transform,
                    ..default()
                },
                vrm: asset_server.load(path),
                ..default()
            });
        }
    }
}
