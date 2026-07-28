use std::{
    collections::{
        HashMap,
        HashSet,
    },
    sync::LazyLock,
};

use bevy::{
    camera::visibility::RenderLayers,
    mesh::{
        Indices,
        VertexAttributeValues,
        morph::MeshMorphWeights,
        skinning::SkinnedMesh,
    },
    prelude::*,
};
use bevy_shader_mtoon::MtoonMaterial;
use serde_vrm::vrm0::BoneName;
pub use serde_vrm::vrm0::FirstPersonFlag;

pub const FIRST_PERSON_LAYER: usize = 7;
pub const THIRD_PERSON_LAYER: usize = 8;

pub static DEFAULT_RENDER_LAYERS: LazyLock<HashMap<FirstPersonFlag, RenderLayers>> =
    LazyLock::new(|| {
        let mut map = HashMap::default();

        map.insert(
            FirstPersonFlag::Auto,
            RenderLayers::from_layers(&[0, FIRST_PERSON_LAYER, THIRD_PERSON_LAYER]),
        );
        map.insert(
            FirstPersonFlag::Both,
            RenderLayers::from_layers(&[0, FIRST_PERSON_LAYER, THIRD_PERSON_LAYER]),
        );
        map.insert(
            FirstPersonFlag::FirstPersonOnly,
            RenderLayers::layer(FIRST_PERSON_LAYER),
        );
        map.insert(
            FirstPersonFlag::ThirdPersonOnly,
            RenderLayers::layer(THIRD_PERSON_LAYER),
        );

        map
    });

/// Splits each VRM mesh into first- and third-person variants and assigns their
/// [`RenderLayers`]. Cameras must render
/// [`FIRST_PERSON_LAYER`]/[`THIRD_PERSON_LAYER`] to see the split meshes.
#[derive(EntityEvent)]
pub struct SetupFirstPerson {
    pub entity:        Entity,
    /// Render layer values to use for each first person flag.
    /// Defaults to [`DEFAULT_RENDER_LAYERS`] if not provided.
    pub render_layers: Option<HashMap<FirstPersonFlag, RenderLayers>>,
}

pub(crate) fn setup_first_person(
    event: On<SetupFirstPerson>,
    bones: Query<(Entity, &BoneName)>,
    mut flags: Query<(
        Entity,
        &mut FirstPersonFlag,
        &Mesh3d,
        Option<&Name>,
        Option<&MeshMaterial3d<StandardMaterial>>,
        Option<&MeshMaterial3d<MtoonMaterial>>,
        Option<&MeshMorphWeights>,
    )>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    parents: Query<&ChildOf>,
    skins: Query<&SkinnedMesh>,
) {
    if bones.is_empty() {
        return;
    }

    let render_layers = event
        .render_layers
        .as_ref()
        .unwrap_or(&DEFAULT_RENDER_LAYERS);

    let (head_ent, _) = bones
        .iter()
        .find(|(e, name)| **name == BoneName::Head && is_child(*e, event.entity, &parents))
        .expect("VRM must have Head bone");

    for (ent, mut flag, mesh_handle, name, standard_material, mtoon_material, morph_weights) in
        &mut flags
    {
        // Skip if not a child of this VRM entity.
        if !is_child(ent, event.entity, &parents) {
            continue;
        }
        // If auto, split the mesh into first-person and third-person variants.
        // Each vertex that is weighted to the head bone gets removed from the
        // first-person variant.
        if *flag == FirstPersonFlag::Auto {
            let Some(mesh) = meshes.get(mesh_handle) else {
                warn!("Mesh not found");
                continue;
            };

            let mut mesh = mesh.clone();

            let Some(VertexAttributeValues::Uint16x4(joints)) =
                mesh.attribute(Mesh::ATTRIBUTE_JOINT_INDEX)
            else {
                continue;
            };

            let Some(VertexAttributeValues::Float32x4(weights)) =
                mesh.attribute(Mesh::ATTRIBUTE_JOINT_WEIGHT)
            else {
                continue;
            };

            let Ok(skin) = skins.get(ent) else {
                continue;
            };

            let mut to_remove = HashSet::<usize>::default();

            for (i, item) in joints.iter().enumerate() {
                for (j, idx) in item.iter().enumerate() {
                    let joint_ent = skin.joints[*idx as usize];

                    if is_child(joint_ent, head_ent, &parents) {
                        let weight = weights[i];
                        let weight = weight[j];

                        if weight > 0.0 {
                            to_remove.insert(i);
                        }
                    }
                }
            }

            let mut to_remove = to_remove.into_iter().collect::<Vec<_>>();
            to_remove.sort_by(|a, b| b.cmp(a));

            if let Some(indices) = mesh.indices_mut() {
                match indices {
                    Indices::U16(vec) => {
                        clean_indices(vec, &to_remove);
                    }
                    Indices::U32(vec) => {
                        clean_indices(vec, &to_remove);
                    }
                }
            }

            let mut new_skin = skin.clone();
            let new_mesh_handle = meshes.add(mesh);

            let new_ent = commands
                .spawn((
                    Transform::default(),
                    Mesh3d(new_mesh_handle),
                    render_layers[&FirstPersonFlag::FirstPersonOnly].clone(),
                ))
                .id();

            if let Some(v) = mtoon_material {
                commands.entity(new_ent).insert(v.clone());
            }

            if let Some(v) = standard_material {
                commands.entity(new_ent).insert(v.clone());
            }

            if let Some(v) = name {
                commands.entity(new_ent).insert(v.clone());
            }

            if let Some(v) = morph_weights {
                commands.entity(new_ent).insert(v.clone());
            }

            for (i, e) in new_skin.joints.iter().enumerate() {
                if *e == ent {
                    new_skin.joints.insert(i, new_ent);
                    break;
                }
            }

            commands.entity(new_ent).insert(new_skin);
            commands.entity(ent).add_child(new_ent);

            *flag = FirstPersonFlag::ThirdPersonOnly;
        }

        commands
            .entity(ent)
            .insert(render_layers[flag.as_ref()].clone());
    }
}

trait ToUsize {
    fn to_usize(self) -> usize;
}

impl ToUsize for u16 {
    fn to_usize(self) -> usize {
        self as usize
    }
}

impl ToUsize for u32 {
    fn to_usize(self) -> usize {
        self as usize
    }
}

/// Remove the specified vertices from the indices.
fn clean_indices<T: Copy + PartialEq + ToUsize>(indices: &mut Vec<T>, vertices: &[usize]) {
    let mut to_remove = Vec::default();

    for (i, chunk) in indices.chunks(3).enumerate() {
        for n in chunk {
            if vertices.contains(&n.to_usize()) {
                to_remove.push(i);
                break;
            }
        }
    }

    for i in to_remove.into_iter().rev() {
        let start = i * 3;
        indices.remove(start);
        indices.remove(start);
        indices.remove(start);
    }
}

/// Walks up the parent tree, searching for a specific Entity.
fn is_child(target_child: Entity, target_parent: Entity, parents: &Query<&ChildOf>) -> bool {
    if target_child == target_parent {
        true
    } else if let Ok(child_of) = parents.get(target_child) {
        is_child(child_of.parent(), target_parent, parents)
    } else {
        false
    }
}
