//! Drag and drop [VRM](https://vrm.dev/) viewer using [bevy_vrm](https://github.com/unavi-xyz/bevy_vrm).

use std::f32::consts::PI;

use bevy::{
    asset::AssetMetaCheck,
    camera::visibility::RenderLayers,
    prelude::*,
    world_serialization::WorldInstanceReady,
};
use bevy_egui::{
    EguiPlugin,
    EguiPrimaryContextPass,
};
use bevy_panorbit_camera::{
    PanOrbitCamera,
    PanOrbitCameraPlugin,
};
use bevy_vrm::{
    VrmInstance,
    VrmPlugins,
    first_person::{
        DEFAULT_RENDER_LAYERS,
        FirstPersonFlag,
        SetupFirstPerson,
    },
    mtoon::MtoonSun,
};
use ui::RenderLayer;

mod draw_spring_bones;
mod move_leg;
mod ui;

pub struct VrmViewerPlugin;

impl Plugin for VrmViewerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::linear_rgb(0.1, 0.1, 0.1)))
            .init_resource::<Settings>()
            .add_plugins((
                DefaultPlugins.set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    #[cfg(not(target_family = "wasm"))]
                    file_path: "../../assets".to_string(),
                    ..default()
                }),
                EguiPlugin::default(),
                PanOrbitCameraPlugin,
                VrmPlugins,
            ))
            .add_systems(Startup, setup)
            .add_observer(setup_first_person)
            .add_systems(EguiPrimaryContextPass, ui::update_ui)
            .add_systems(
                Update,
                (
                    draw_spring_bones::draw_spring_bones,
                    draw_spring_bones::move_avatar,
                    load_model,
                    move_leg::move_leg,
                    read_dropped_files,
                    set_render_layers,
                ),
            );
    }
}

#[derive(Resource, Default)]
struct Settings {
    pub draw_spring_bones: bool,
    pub model:             String,
    pub move_avatar:       bool,
    pub move_leg:          bool,
    pub render_layer:      RenderLayer,
}

fn setup(mut commands: Commands, mut settings: ResMut<Settings>) {
    commands.spawn((
        Transform::from_xyz(1.0, 2.0, 5.0),
        PanOrbitCamera {
            focus: Vec3::new(0.0, 0.8, 0.0),
            ..default()
        },
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: 10_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_rotation_x(-PI / 3.0)),
        MtoonSun,
    ));

    let mut transform = Transform::default();
    transform.rotate_y(PI);

    settings.model = "alicia.vrm".to_string();
}

fn set_render_layers(
    cameras: Query<Entity, With<Camera>>,
    mut commands: Commands,
    mut prev: Local<FirstPersonFlag>,
    settings: Res<Settings>,
) {
    for entity in cameras.iter() {
        let flag = match settings.render_layer {
            RenderLayer::FirstPerson => FirstPersonFlag::FirstPersonOnly,
            RenderLayer::ThirdPerson => FirstPersonFlag::ThirdPersonOnly,
        };

        if flag != *prev {
            *prev = flag;

            let layers = RenderLayers::layer(0).union(&DEFAULT_RENDER_LAYERS[&flag]);
            commands.entity(entity).insert(layers);
        }
    }
}

fn setup_first_person(
    event: On<WorldInstanceReady>,
    mut commands: Commands,
    vrms: Query<(), With<VrmInstance>>,
) {
    let entity = event.entity;

    if vrms.contains(entity) {
        commands.entity(entity).trigger(|entity| SetupFirstPerson {
            entity,
            render_layers: None,
        });
    }
}

fn load_model(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut prev: Local<String>,
    mut vrms: Query<Entity, With<VrmInstance>>,
    settings: Res<Settings>,
) {
    if prev.as_str() == settings.model.as_str() {
        return;
    }

    if let Ok(entity) = vrms.single_mut() {
        commands.entity(entity).despawn();
    }

    let mut transform = Transform::default();
    transform.rotate_y(PI);

    commands.spawn((
        transform,
        VrmInstance(asset_server.load(settings.model.clone())),
    ));

    prev.clone_from(&settings.model);
}

fn read_dropped_files(mut events: MessageReader<FileDragAndDrop>, mut settings: ResMut<Settings>) {
    for event in events.read() {
        if let FileDragAndDrop::DroppedFile { path_buf, .. } = event {
            #[cfg(target_family = "wasm")]
            let path = String::from(path_buf.to_str().expect("File path must be valid UTF-8"));
            #[cfg(not(target_family = "wasm"))]
            let path = bevy::asset::AssetPath::from_path(path_buf.as_path());

            info!("DroppedFile: {}", path);
            settings.model = path.to_string();
        }
    }
}
