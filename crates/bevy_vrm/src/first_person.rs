use std::sync::LazyLock;

use bevy::{
    prelude::*,
    render::{
        mesh::{morph::MeshMorphWeights, skinning::SkinnedMesh, VertexAttributeValues},
        view::RenderLayers,
    },
    utils::{HashMap, HashSet},
};
use bevy_shader_mtoon::MtoonMaterial;
use serde_vrm::vrm0::BoneName;

pub use serde_vrm::vrm0::FirstPersonFlag;

pub const FIRST_PERSON_LAYER: usize = 7;
pub const THIRD_PERSON_LAYER: usize = 8;

pub static RENDER_LAYERS: LazyLock<HashMap<FirstPersonFlag, RenderLayers>> = LazyLock::new(|| {
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

macro_rules! call_vertex_vec {
    ($enum_val:expr, $index:expr, $method:ident) => {
        match $enum_val {
            VertexAttributeValues::Float32(ref mut vec) => {
                vec.$method($index);
            }
            VertexAttributeValues::Sint32(ref mut vec) => {
                vec.$method($index);
            }
            VertexAttributeValues::Uint32(ref mut vec) => {
                vec.$method($index);
            }
            VertexAttributeValues::Float32x2(ref mut vec) => {
                vec.$method($index);
            }
            VertexAttributeValues::Sint32x2(ref mut vec) => {
                vec.$method($index);
            }
            VertexAttributeValues::Uint32x2(ref mut vec) => {
                vec.$method($index);
            }
            VertexAttributeValues::Float32x3(ref mut vec) => {
                vec.$method($index);
            }
            VertexAttributeValues::Sint32x3(ref mut vec) => {
                vec.$method($index);
            }
            VertexAttributeValues::Uint32x3(ref mut vec) => {
                vec.$method($index);
            }
            VertexAttributeValues::Float32x4(ref mut vec) => {
                vec.$method($index);
            }
            VertexAttributeValues::Sint32x4(ref mut vec) => {
                vec.$method($index);
            }
            VertexAttributeValues::Uint32x4(ref mut vec) => {
                vec.$method($index);
            }
            VertexAttributeValues::Sint16x2(ref mut vec) => {
                vec.$method($index);
            }
            VertexAttributeValues::Snorm16x2(ref mut vec) => {
                vec.$method($index);
            }
            VertexAttributeValues::Uint16x2(ref mut vec) => {
                vec.$method($index);
            }
            VertexAttributeValues::Unorm16x2(ref mut vec) => {
                vec.$method($index);
            }
            VertexAttributeValues::Sint16x4(ref mut vec) => {
                vec.$method($index);
            }
            VertexAttributeValues::Snorm16x4(ref mut vec) => {
                vec.$method($index);
            }
            VertexAttributeValues::Uint16x4(ref mut vec) => {
                vec.$method($index);
            }
            VertexAttributeValues::Unorm16x4(ref mut vec) => {
                vec.$method($index);
            }
            VertexAttributeValues::Sint8x2(ref mut vec) => {
                vec.$method($index);
            }
            VertexAttributeValues::Snorm8x2(ref mut vec) => {
                vec.$method($index);
            }
            VertexAttributeValues::Uint8x2(ref mut vec) => {
                vec.$method($index);
            }
            VertexAttributeValues::Unorm8x2(ref mut vec) => {
                vec.$method($index);
            }
            VertexAttributeValues::Sint8x4(ref mut vec) => {
                vec.$method($index);
            }
            VertexAttributeValues::Snorm8x4(ref mut vec) => {
                vec.$method($index);
            }
            VertexAttributeValues::Uint8x4(ref mut vec) => {
                vec.$method($index);
            }
            VertexAttributeValues::Unorm8x4(ref mut vec) => {
                vec.$method($index);
            }
        }
    };
}

#[derive(Event)]
pub struct SetupFirstPerson(pub Entity);

pub(crate) fn handle_setup_events(
    mut events: EventReader<SetupFirstPerson>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut scene_assets: ResMut<Assets<Scene>>,
    scenes: Query<&Handle<Scene>>,
) {
    for event in events.read() {
        let Ok(handle) = scenes.get(event.0) else {
            warn!("SetupFirstPerson event must be called on a Handle<Scene>");
            continue;
        };

        let Some(scene) = scene_assets.get_mut(handle) else {
            warn!("Scene not found");
            continue;
        };

        let mut bones = scene.world.query::<(Entity, &BoneName)>();
        let mut flags = scene
            .world
            .query::<(Entity, &FirstPersonFlag, &Handle<Mesh>)>();
        let mut parents = scene.world.query::<&Parent>();
        let mut skins = scene.world.query::<&SkinnedMesh>();
        let mut standard_materials = scene.world.query::<&Handle<StandardMaterial>>();
        let mut mtoon_materials = scene.world.query::<&Handle<MtoonMaterial>>();
        let mut names = scene.world.query::<&Name>();
        let mut morph_weights = scene.world.query::<&MeshMorphWeights>();

        let (head_ent, _) = bones
            .iter(&scene.world)
            .find(|(_, name)| **name == BoneName::Head)
            .unwrap();

        for (ent, mut flag, mesh_handle) in flags
            .iter(&scene.world)
            .map(|(e, f, m)| (e, *f, m.clone()))
            .collect::<Vec<_>>()
        {
            // If auto, split the mesh into first-person and third-person variants.
            // Each vertex that is weighted to the head bone gets removed from the first-person variant.
            if flag == FirstPersonFlag::Auto {
                let Some(mesh) = meshes.get(&mesh_handle) else {
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

                let Ok(skin) = skins.get(&scene.world, ent) else {
                    continue;
                };

                let mut to_remove = HashSet::default();

                for (i, item) in joints.iter().enumerate() {
                    for (j, idx) in item.iter().enumerate() {
                        let joint_ent = skin.joints[*idx as usize];

                        if is_child(joint_ent, head_ent, &mut parents, &scene.world) {
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

                for (_, values) in mesh.attributes_mut() {
                    for i in to_remove.iter().copied() {
                        call_vertex_vec!(values, i, remove);
                    }
                }

                // TODO: Optimize mesh by removing un-needed indices?

                let mut new_skin = skin.clone();
                let new_mesh_handle = meshes.add(mesh);

                let new_ent = scene
                    .world
                    .spawn((
                        SpatialBundle::default(),
                        new_mesh_handle,
                        RENDER_LAYERS[&FirstPersonFlag::FirstPersonOnly].clone(),
                    ))
                    .id();

                if let Ok(v) = mtoon_materials.get(&scene.world, ent).cloned() {
                    scene.world.entity_mut(new_ent).insert(v);
                }

                if let Ok(v) = standard_materials.get(&scene.world, ent).cloned() {
                    scene.world.entity_mut(new_ent).insert(v);
                }

                if let Ok(v) = names.get(&scene.world, ent).cloned() {
                    scene.world.entity_mut(new_ent).insert(v);
                }

                if let Ok(v) = morph_weights.get(&scene.world, ent).cloned() {
                    scene.world.entity_mut(new_ent).insert(v);
                }

                for (i, e) in new_skin.joints.iter().enumerate() {
                    if *e == ent {
                        new_skin.joints.insert(i, new_ent);
                        break;
                    }
                }

                scene.world.entity_mut(new_ent).insert(new_skin);

                scene.world.entity_mut(ent).add_child(new_ent);

                flag = FirstPersonFlag::ThirdPersonOnly;
            }

            scene
                .world
                .entity_mut(ent)
                .insert(RENDER_LAYERS[&flag].clone());
        }
    }
}

/// Walks up the parent tree, searching for a specific Entity.
fn is_child(
    target_child: Entity,
    target_parent: Entity,
    parents: &mut QueryState<&Parent>,
    world: &World,
) -> bool {
    if target_child == target_parent {
        true
    } else if let Ok(parent) = parents.get(world, target_child) {
        is_child(parent.get(), target_parent, parents, world)
    } else {
        false
    }
}
