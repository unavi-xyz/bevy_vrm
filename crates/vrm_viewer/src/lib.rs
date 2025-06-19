//! Drag and drop [VRM](https://vrm.dev/) viewer using [bevy_vrm](https://github.com/unavi-xyz/bevy_vrm).

use std::f32::consts::PI;

use bevy::{asset::AssetMetaCheck, prelude::*, render::view::RenderLayers};
use bevy_egui::EguiPlugin;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_vrm::{
    VrmBundle, VrmPlugins,
    first_person::{FirstPersonFlag, RENDER_LAYERS, SetupFirstPerson},
    loader::Vrm,
    mtoon::MtoonSun,
};
use ui::RenderLayer;

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

        app.insert_resource(ClearColor(Color::linear_rgb(0.1, 0.1, 0.1)))
            .init_resource::<Settings>()
            .add_plugins((
                DefaultPlugins.set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                }),
                EguiPlugin,
                PanOrbitCameraPlugin,
                VrmPlugins,
            ))
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    draw_spring_bones::draw_spring_bones,
                    draw_spring_bones::move_avatar,
                    load_model,
                    move_leg::move_leg,
                    read_dropped_files,
                    set_render_layers,
                    setup_first_person,
                    ui::update_ui,
                ),
            );
    }
}

#[derive(Resource, Default)]
struct Settings {
    pub draw_spring_bones: bool,
    pub model: String,
    pub move_avatar: bool,
    pub move_leg: bool,
    pub render_layer: RenderLayer,
}

fn setup(mut commands: Commands, mut settings: ResMut<Settings>) {
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

            let layers = RenderLayers::layer(0).union(&RENDER_LAYERS[&flag]);
            commands.entity(entity).insert(layers);
        }
    }
}

fn setup_first_person(
    mut events: EventReader<AssetEvent<Vrm>>,
    mut writer: EventWriter<SetupFirstPerson>,
    vrms: Query<(Entity, &Handle<Vrm>)>,
) {
    for event in events.read() {
        if let AssetEvent::LoadedWithDependencies { id } = event {
            let (ent, _) = vrms.iter().find(|(_, handle)| handle.id() == *id).unwrap();
            writer.send(SetupFirstPerson(ent));
        }
    }
}

fn load_model(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut prev: Local<String>,
    mut vrms: Query<Entity, With<Handle<Vrm>>>,
    settings: Res<Settings>,
) {
    if prev.as_str() == settings.model.as_str() {
        return;
    }

    if let Ok(entity) = vrms.get_single_mut() {
        commands.entity(entity).despawn_recursive();
    }

    let mut transform = Transform::default();
    transform.rotate_y(PI);

    commands.spawn(VrmBundle {
        scene_bundle: SceneBundle {
            transform,
            ..default()
        },
        vrm: asset_server.load(settings.model.clone()),
        ..default()
    });

    *prev = settings.model.clone();
}

fn read_dropped_files(mut events: EventReader<FileDragAndDrop>, mut settings: ResMut<Settings>) {
    for event in events.read() {
        if let FileDragAndDrop::DroppedFile { path_buf, .. } = event {
            #[cfg(target_family = "wasm")]
            let path = String::from(path_buf.to_str().unwrap());
            #[cfg(not(target_family = "wasm"))]
            let path = bevy::asset::AssetPath::from_path(path_buf.as_path());

            info!("DroppedFile: {}", path);
            settings.model = path.to_string();
        }
    }
}
