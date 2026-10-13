//! Splitting of avatar meshes into first- and third-person render layers.

use std::collections::HashSet;

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
use bevy_shader_mtoon::{
    MtoonMaterial,
    OutlineSync,
};
use serde_vrm::vrm1::vrmc_vrm::{
    first_person::FirstPersonType,
    humanoid::HumanBoneName,
};

pub const FIRST_PERSON_LAYER: usize = 7;
pub const THIRD_PERSON_LAYER: usize = 8;

#[derive(Clone, Debug)]
pub struct FirstPersonLayers {
    pub both:              RenderLayers,
    pub first_person_only: RenderLayers,
    pub third_person_only: RenderLayers,
}

impl Default for FirstPersonLayers {
    fn default() -> Self {
        Self {
            both:              RenderLayers::from_layers(&[
                0,
                FIRST_PERSON_LAYER,
                THIRD_PERSON_LAYER,
            ]),
            first_person_only: RenderLayers::layer(FIRST_PERSON_LAYER),
            third_person_only: RenderLayers::layer(THIRD_PERSON_LAYER),
        }
    }
}

impl FirstPersonLayers {
    /// [`FirstPersonType::Auto`] shares the layers of
    /// [`FirstPersonType::Both`].
    #[must_use]
    pub const fn get(&self, ty: FirstPersonType) -> &RenderLayers {
        match ty {
            FirstPersonType::Auto | FirstPersonType::Both => &self.both,
            FirstPersonType::FirstPersonOnly => &self.first_person_only,
            FirstPersonType::ThirdPersonOnly => &self.third_person_only,
        }
    }
}

/// Splits each VRM mesh into first- and third-person variants and assigns their
/// [`RenderLayers`]. Cameras must render
/// [`FIRST_PERSON_LAYER`]/[`THIRD_PERSON_LAYER`] to see the split meshes.
#[derive(EntityEvent)]
pub struct SetupFirstPerson {
    pub entity: Entity,
    pub layers: FirstPersonLayers,
}

pub(crate) fn setup_first_person(
    event: On<SetupFirstPerson>,
    bones: Query<(Entity, &HumanBoneName)>,
    mut flags: Query<(
        Entity,
        &mut FirstPersonType,
        &Mesh3d,
        Option<&Name>,
        Option<&MeshMaterial3d<StandardMaterial>>,
        Option<&MeshMaterial3d<MtoonMaterial>>,
        Option<&MeshMorphWeights>,
        Option<&OutlineSync>,
    )>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    parents: Query<&ChildOf>,
    skins: Query<&SkinnedMesh>,
) {
    if bones.is_empty() {
        return;
    }

    let layers = &event.layers;

    let Some((head_ent, _)) = bones
        .iter()
        .find(|(e, name)| **name == HumanBoneName::Head && is_child(*e, event.entity, &parents))
    else {
        warn!("VRM has no Head bone, skipping first person setup");
        return;
    };

    for (
        ent,
        mut flag,
        mesh_handle,
        name,
        standard_material,
        mtoon_material,
        morph_weights,
        outline_sync,
    ) in &mut flags
    {
        if !is_child(ent, event.entity, &parents) {
            continue;
        }
        // Auto splits into a head-removed first-person variant plus the full
        // third-person original, each on its own render layer.
        if *flag == FirstPersonType::Auto {
            'split: {
                let Some(mesh) = meshes.get(mesh_handle) else {
                    warn!("Mesh not found");
                    break 'split;
                };
                let Ok(skin) = skins.get(ent) else {
                    break 'split;
                };
                let Some(first_person) = strip_head_vertices(mesh, skin, head_ent, &parents) else {
                    break 'split;
                };

                let new_ent = commands
                    .spawn((
                        Transform::default(),
                        Mesh3d(meshes.add(first_person)),
                        layers.first_person_only.clone(),
                    ))
                    .id();

                insert_cloned(&mut commands, new_ent, mtoon_material);
                insert_cloned(&mut commands, new_ent, standard_material);
                insert_cloned(&mut commands, new_ent, name);
                insert_cloned(&mut commands, new_ent, morph_weights);
                insert_cloned(&mut commands, new_ent, outline_sync);

                let mut new_skin = skin.clone();
                for (i, e) in new_skin.joints.iter().enumerate() {
                    if *e == ent {
                        new_skin.joints.insert(i, new_ent);
                        break;
                    }
                }

                commands.entity(new_ent).insert(new_skin);
                commands.entity(ent).add_child(new_ent);

                *flag = FirstPersonType::ThirdPersonOnly;
            }
        }

        commands.entity(ent).insert(layers.get(*flag).clone());
    }
}

fn insert_cloned<T: Component + Clone>(
    commands: &mut Commands,
    entity: Entity,
    component: Option<&T>,
) {
    if let Some(component) = component {
        commands.entity(entity).insert(component.clone());
    }
}

/// Clones a mesh with every triangle touching a head-weighted vertex removed.
fn strip_head_vertices(
    mesh: &Mesh,
    skin: &SkinnedMesh,
    head_ent: Entity,
    parents: &Query<&ChildOf>,
) -> Option<Mesh> {
    let Some(VertexAttributeValues::Uint16x4(joints)) = mesh.attribute(Mesh::ATTRIBUTE_JOINT_INDEX)
    else {
        return None;
    };
    let Some(VertexAttributeValues::Float32x4(weights)) =
        mesh.attribute(Mesh::ATTRIBUTE_JOINT_WEIGHT)
    else {
        return None;
    };

    let mut to_remove = HashSet::<usize>::default();
    for (i, (item, item_weights)) in joints.iter().zip(weights).enumerate() {
        for (idx, weight) in item.iter().zip(item_weights) {
            let Some(&joint_ent) = skin.joints.get(*idx as usize) else {
                continue;
            };
            if is_child(joint_ent, head_ent, parents) && *weight > 0.0 {
                to_remove.insert(i);
            }
        }
    }

    let mut mesh = mesh.clone();
    if let Some(indices) = mesh.indices_mut() {
        match indices {
            Indices::U16(vec) => clean_indices(vec, &to_remove),
            Indices::U32(vec) => clean_indices(vec, &to_remove),
        }
    }
    Some(mesh)
}

/// Keeps only the triangles that touch none of the removed vertices.
fn clean_indices<T: Copy + Into<u32>>(indices: &mut Vec<T>, removed: &HashSet<usize>) {
    *indices = indices
        .as_chunks::<3>()
        .0
        .iter()
        .filter(|tri| {
            tri.iter()
                .all(|i| !removed.contains(&(Into::<u32>::into(*i) as usize)))
        })
        .flatten()
        .copied()
        .collect();
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
