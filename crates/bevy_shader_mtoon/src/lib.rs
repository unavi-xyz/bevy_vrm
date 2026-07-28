//! Bevy plugin implementing the [MToon](https://vrm.dev/en/univrm/shaders/shader_mtoon.html) shader.

use std::collections::HashMap;

use bevy::{
    asset::{
        load_internal_asset,
        uuid_handle,
    },
    camera::visibility::RenderLayers,
    mesh::{
        VertexAttributeValues,
        morph::MeshMorphWeights,
        skinning::SkinnedMesh,
    },
    prelude::*,
};

mod outline;
mod shader;

pub use outline::OutlineMaterial;
pub use shader::{
    MtoonMaterial,
    OutlineMode as VrmOutlineMode,
};

const SHADER_HANDLE: Handle<Shader> = uuid_handle!("88901104-e489-4263-b974-94885e37a3a7");
const OUTLINE_SHADER_HANDLE: Handle<Shader> = uuid_handle!("2f1c3d6e-9a4b-4f8c-8c2a-1b7e4d5a6c90");

#[derive(Default)]
pub struct MtoonPlugin;

impl Plugin for MtoonPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(app, SHADER_HANDLE, "mtoon.wgsl", Shader::from_wgsl);
        load_internal_asset!(
            app,
            OUTLINE_SHADER_HANDLE,
            "outline.wgsl",
            Shader::from_wgsl
        );

        app.register_type::<OutlineSync>()
            .add_plugins((
                MaterialPlugin::<MtoonMaterial>::default(),
                MaterialPlugin::<OutlineMaterial>::default(),
            ))
            .add_systems(Update, (update_mtoon_shader, sync_outline));
    }
}

#[derive(Bundle, Clone, Default)]
pub struct MtoonBundle {
    pub mtoon:        MeshMaterial3d<MtoonMaterial>,
    pub outline_sync: OutlineSync,
}

/// Marks a [`DirectionalLight`] to be used for shading within the `MToon`
/// shader. Only a single [`MtoonSun`] is allowed.
#[derive(Component)]
pub struct MtoonSun;

fn update_mtoon_shader(
    mut materials: ResMut<Assets<MtoonMaterial>>,
    mut events: MessageReader<AssetEvent<MtoonMaterial>>,
    sun: Query<(&GlobalTransform, &DirectionalLight), With<MtoonSun>>,
    changed_sun: Query<
        (),
        (
            With<MtoonSun>,
            Or<(Changed<GlobalTransform>, Changed<DirectionalLight>)>,
        ),
    >,
) {
    let Ok((transform, light)) = sun.single() else {
        events.clear();
        return;
    };

    let light_dir = transform.back().as_vec3();
    let light_color = light.color;

    if !changed_sun.is_empty() {
        events.clear();
        for (_, material) in materials.iter_mut() {
            material.light_dir = light_dir;
            material.light_color = light_color;
        }
        return;
    }

    let added = events
        .read()
        .filter_map(|event| match event {
            AssetEvent::Added { id } => Some(*id),
            _ => None,
        })
        .collect::<Vec<_>>();

    for id in added {
        if let Some(mut material) = materials.get_mut(id) {
            material.light_dir = light_dir;
            material.light_color = light_color;
        }
    }
}

/// Syncs an outline with the outline properties of an entity's
/// [`MtoonMaterial`]. The outline renders on a child entity so it coexists with
/// the base material.
#[derive(Component, Clone, Default, Reflect)]
#[reflect(Component)]
pub struct OutlineSync;

/// Points a mesh at its outline child spawned by [`sync_outline`].
#[derive(Component)]
struct OutlineChild(Entity);

fn sync_outline(
    mut commands: Commands,
    mut outlines: ResMut<Assets<OutlineMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mtoons: Res<Assets<MtoonMaterial>>,
    query: Query<
        (
            Entity,
            &MeshMaterial3d<MtoonMaterial>,
            &Mesh3d,
            Option<&OutlineChild>,
            Option<&SkinnedMesh>,
            Option<&MeshMorphWeights>,
            Option<&RenderLayers>,
        ),
        With<OutlineSync>,
    >,
    child_materials: Query<&MeshMaterial3d<OutlineMaterial>>,
) {
    for (entity, mtoon_handle, mesh, child, skin, morph, layers) in &query {
        let Some(mtoon) = mtoons.get(mtoon_handle.id()) else {
            continue;
        };

        if mtoon.outline_mode == VrmOutlineMode::None {
            if let Some(child) = child {
                commands.entity(child.0).despawn();
                commands.entity(entity).remove::<OutlineChild>();
            }
            continue;
        }

        let params = OutlineMaterial::from_mtoon(mtoon);

        if let Some(child) = child {
            if let Ok(material) = child_materials.get(child.0)
                && let Some(mut existing) = outlines.get_mut(material.id())
                && *existing != params
            {
                *existing = params;
            }
            match layers {
                Some(layers) => {
                    commands.entity(child.0).insert(layers.clone());
                }
                None => {
                    commands.entity(child.0).remove::<RenderLayers>();
                }
            }
            continue;
        }

        let Some(outline_mesh) = meshes.get(mesh.id()).and_then(smooth_outline_mesh) else {
            continue;
        };

        let mesh_handle = meshes.add(outline_mesh);
        let handle = outlines.add(params);
        let mut child_commands = commands.spawn((
            Mesh3d(mesh_handle),
            MeshMaterial3d(handle),
            Transform::default(),
        ));
        if let Some(skin) = skin {
            child_commands.insert(skin.clone());
        }
        if let Some(morph) = morph {
            child_commands.insert(morph.clone());
        }
        if let Some(layers) = layers {
            child_commands.insert(layers.clone());
        }
        let child_id = child_commands.id();
        commands
            .entity(entity)
            .insert(OutlineChild(child_id))
            .add_child(child_id);
    }
}

/// Clones a mesh, replacing its normals with position-welded averaged normals
/// so an inverted-hull outline extrudes without splitting at hard edges.
fn smooth_outline_mesh(mesh: &Mesh) -> Option<Mesh> {
    let Some(VertexAttributeValues::Float32x3(positions)) =
        mesh.attribute(Mesh::ATTRIBUTE_POSITION)
    else {
        return None;
    };
    let Some(VertexAttributeValues::Float32x3(normals)) = mesh.attribute(Mesh::ATTRIBUTE_NORMAL)
    else {
        return None;
    };
    if positions.len() != normals.len() {
        return None;
    }

    let quantize = |p: &[f32; 3]| {
        [
            (p[0] * 1024.0).round() as i32,
            (p[1] * 1024.0).round() as i32,
            (p[2] * 1024.0).round() as i32,
        ]
    };

    let mut sums: HashMap<[i32; 3], Vec3> = HashMap::new();
    for (p, n) in positions.iter().zip(normals) {
        *sums.entry(quantize(p)).or_default() += Vec3::from_array(*n);
    }

    let smoothed = positions
        .iter()
        .zip(normals)
        .map(|(p, n)| {
            let sum = sums[&quantize(p)];
            if sum.length_squared() > 1.0e-8 {
                sum.normalize().to_array()
            } else {
                *n
            }
        })
        .collect::<Vec<_>>();

    let mut outline = mesh.clone();
    outline.insert_attribute(Mesh::ATTRIBUTE_NORMAL, smoothed);
    Some(outline)
}
