//! Drag and drop [VRM](https://vrm.dev/) viewer using [bevy_vrm](https://github.com/unavi-xyz/bevy_vrm).

use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_vrm::{loader::Vrm, mtoon::MtoonSun, VrmBundle, VrmPlugin};

pub struct VrmViewerPlugin;

impl Plugin for VrmViewerPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(target_family = "wasm")]
        {
            app.add_plugins(bevy_web_file_drop::WebFileDropPlugin);
        }

        app.insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
            .add_plugins((DefaultPlugins, EguiPlugin, PanOrbitCameraPlugin, VrmPlugin))
            .add_systems(Startup, setup)
            .add_systems(Update, (update_ui, read_dropped_files));
    }
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

fn update_ui(mut contexts: EguiContexts) {
    bevy_egui::egui::Window::new("VRM Viewer").show(contexts.ctx_mut(), |ui| {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;

                ui.label("Loads ");
                ui.hyperlink_to("VRM", "https://vrm.dev/en");
                ui.label(" avatars using ");
                ui.hyperlink_to("bevy_vrm", "https://github.com/unavi-xyz/bevy_vrm");
                ui.label(", a plugin for the ");
                ui.hyperlink_to("Bevy", "https://bevyengine.org");
                ui.label(" game engine.");
            });

            ui.label("Drop a .vrm file into the window to load it.");

            ui.separator();

            ui.vertical_centered(|ui| {
                ui.horizontal(|ui| {
                    ui.hyperlink_to("[github]", "https://github.com/unavi-xyz/bevy_vrm");
                });
            });
        });
    });
}

fn read_dropped_files(
    mut events: EventReader<FileDragAndDrop>,
    asset_server: Res<AssetServer>,
    mut vrms: Query<&mut Handle<Vrm>>,
) {
    for event in events.read() {
        if let FileDragAndDrop::DroppedFile { path_buf, .. } = event {
            #[cfg(target_family = "wasm")]
            let path = String::from(path_buf.to_str().unwrap());
            #[cfg(not(target_family = "wasm"))]
            let path = bevy::asset::AssetPath::from_path(path_buf.as_path());

            info!("DroppedFile: {}", path);

            let mut vrm = vrms.single_mut();
            *vrm = asset_server.load(path);
        }
    }
}
