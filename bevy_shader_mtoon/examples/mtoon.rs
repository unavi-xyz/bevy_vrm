use std::f32::consts::PI;

use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
};

use bevy_shader_mtoon::{MtoonMainCamera, MtoonMaterial, MtoonPlugin, MtoonSun};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            MtoonPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, rotate)
        .run();
}

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut mtoon_materials: ResMut<Assets<MtoonMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 10.0, 15.0)
                .looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
            ..default()
        },
        MtoonMainCamera,
    ));

    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: true,
                illuminance: 10_000.,
                ..default()
            },
            transform: Transform::from_xyz(2.0, 2.0, 2.0),
            ..default()
        },
        MtoonSun,
    ));

    let mtoon_textured = mtoon_materials.add(MtoonMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        shading_shift_factor: 0.5,
        ..default()
    });

    let mtoon = mtoon_materials.add(MtoonMaterial {
        base_color: Color::BISQUE,
        shade_color: Color::SALMON,
        ..default()
    });

    let shapes = [
        meshes.add(Cuboid::default()),
        meshes.add(Capsule3d::default()),
        meshes.add(Torus::default()),
        meshes.add(Cylinder::default()),
        meshes.add(Sphere::default()),
    ];

    let num_shapes = shapes.len();

    // Spacing between shapes
    const X_EXTENT: f32 = 10.0;

    for (i, mesh) in shapes.into_iter().enumerate() {
        // Texture
        commands.spawn(MaterialMeshBundle {
            mesh: mesh.clone(),
            material: mtoon_textured.clone(),
            transform: Transform::from_xyz(
                -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
                2.0,
                3.0,
            )
            .with_rotation(Quat::from_rotation_x(-PI / 4.)),
            ..default()
        });

        // Without texture
        commands.spawn(MaterialMeshBundle {
            mesh,
            material: mtoon.clone(),
            transform: Transform::from_xyz(
                -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
                2.0,
                -3.0,
            )
            .with_rotation(Quat::from_rotation_x(-PI / 4.0)),
            ..default()
        });
    }

    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default()),
        material: materials.add(StandardMaterial::from(Color::SILVER)),
        ..default()
    });
}

fn rotate(time: Res<Time>, mut query: Query<&mut Transform, With<Handle<MtoonMaterial>>>) {
    for mut transform in query.iter_mut() {
        transform.rotate(Quat::from_rotation_y(time.delta_seconds() / 2.0));
    }
}

fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    )
}
