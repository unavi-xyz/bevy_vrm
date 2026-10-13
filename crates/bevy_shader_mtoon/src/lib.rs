//! Bevy plugin implementing the [MToon](https://vrm.dev/en/univrm/shaders/shader_mtoon.html) shader.

// Bevy`s `AsBindGroup` needs higher limit
#![recursion_limit = "256"]

use std::collections::HashMap;

use bevy::{
    camera::visibility::RenderLayers,
    mesh::{
        VertexAttributeValues,
        morph::MeshMorphWeights,
        skinning::SkinnedMesh,
    },
    prelude::*,
    shader::load_shader_library,
};

mod outline;
mod shader;

pub use outline::OutlineMaterial;
pub use shader::{
    MtoonMaterial,
    OutlineMode as VrmOutlineMode,
};

/// Importable from other WESL shaders as `bevy_shader_mtoon::mtoon`.
const SHADER_PATH: &str = "embedded://bevy_shader_mtoon/mtoon.wesl";
const OUTLINE_SHADER_PATH: &str = "embedded://bevy_shader_mtoon/outline.wesl";

#[derive(Default)]
pub struct MtoonPlugin;

impl Plugin for MtoonPlugin {
    fn build(&self, app: &mut App) {
        load_shader_library!(app, "mtoon.wesl");
        load_shader_library!(app, "outline.wesl");

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
    outline_children: Query<(&MeshMaterial3d<OutlineMaterial>, Option<&RenderLayers>)>,
) {
    for (entity, mtoon_handle, mesh, child, skin, morph, layers) in &query {
        let Some(mtoon) = mtoons.get(mtoon_handle.id()) else {
            continue;
        };

        if mtoon.outline_mode == VrmOutlineMode::None {
            if let Some(child) = child {
                commands.entity(child.0).try_despawn();
                commands.entity(entity).remove::<OutlineChild>();
            }
            continue;
        }

        let params = OutlineMaterial::from_mtoon(mtoon);

        if let Some(child) = child {
            let Ok((material, child_layers)) = outline_children.get(child.0) else {
                continue;
            };
            if outlines
                .get(material.id())
                .is_some_and(|existing| *existing != params)
                && let Some(mut existing) = outlines.get_mut(material.id())
            {
                *existing = params;
            }
            // The child dies with its parent, which may be despawned later this
            // frame.
            if layers != child_layers {
                match layers {
                    Some(layers) => {
                        commands.entity(child.0).try_insert(layers.clone());
                    }
                    None => {
                        commands.entity(child.0).try_remove::<RenderLayers>();
                    }
                }
            }
            continue;
        }

        let Some(outline_mesh) = meshes.get(mesh.id()).and_then(smooth_outline_mesh) else {
            continue;
        };

        let mesh_handle = meshes.add(outline_mesh);
        let handle = outlines.add(params);
        let skin = skin.cloned();
        let morph = morph.cloned();
        let layers = layers.cloned();

        // Spawned only if the mesh still exists when the command applies, so a
        // mesh despawned this frame leaves no orphaned outline behind.
        commands
            .entity(entity)
            .queue_silenced(move |mut parent: EntityWorldMut| {
                let child = parent.world_scope(|world| {
                    let mut child = world.spawn((
                        Mesh3d(mesh_handle),
                        MeshMaterial3d(handle),
                        Transform::default(),
                    ));
                    if let Some(skin) = skin {
                        child.insert(skin);
                    }
                    if let Some(morph) = morph {
                        child.insert(morph);
                    }
                    if let Some(layers) = layers {
                        child.insert(layers);
                    }
                    child.id()
                });
                parent.insert(OutlineChild(child)).add_child(child);
            });
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
