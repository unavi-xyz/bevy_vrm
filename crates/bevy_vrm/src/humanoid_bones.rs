use bevy::{prelude::*, scene::SceneInstance};
use gltf_kun::graph::{
    gltf::{GltfDocument, GltfWeight},
    ByteNode, Extensions, Weight,
};

use crate::{loader::Vrm, HumanoidBones};

#[derive(Component)]
pub struct HumanoidBonesInitialized;

pub fn set_humanoid_bones(
    mut commands: Commands,
    mut vrm: Query<
        (Entity, &mut HumanoidBones, &Handle<Vrm>, &SceneInstance),
        Without<HumanoidBonesInitialized>,
    >,
    names: Query<(Entity, &Name)>,
    scene_manager: Res<SceneSpawner>,
    vrms: Res<Assets<Vrm>>,
) {
    for (entity, mut humanoid_bones, handle, instance) in vrm.iter_mut() {
        if !scene_manager.instance_is_ready(**instance) {
            continue;
        }

        if let Some(vrm) = vrms.get(handle) {
            commands.entity(entity).insert(HumanoidBonesInitialized);

            let graph = &vrm.gltf.graph;

            let doc = match graph.node_indices().find(|n| {
                let weight = graph.node_weight(*n);
                matches!(weight, Some(Weight::Gltf(GltfWeight::Document)))
            }) {
                Some(doc) => GltfDocument(doc),
                None => continue,
            };

            let ext = match doc.get_extension::<gltf_kun_vrm::vrm0::Vrm>(graph) {
                Some(ext) => ext,
                None => continue,
            };

            for bone in ext.human_bones(graph) {
                let node = match bone.node(graph) {
                    Some(n) => n,
                    None => continue,
                };

                let weight = bone.read(graph);

                let bone_name = match weight.name {
                    Some(b) => b,
                    None => continue,
                };

                let node_handle = match vrm.gltf.node_handles.get(&node) {
                    Some(handle) => handle.clone(),
                    None => continue,
                };

                let node_name = vrm.gltf.named_nodes.iter().find_map(|(name, n)| {
                    if *n == node_handle {
                        Some(name.clone())
                    } else {
                        None
                    }
                });

                let node_name = match node_name {
                    Some(n) => n,
                    None => continue,
                };

                let node_entity = match names.iter().find_map(|(entity, name)| {
                    if name.as_str() == node_name.as_str() {
                        Some(entity)
                    } else {
                        None
                    }
                }) {
                    Some(entity) => entity,
                    None => {
                        warn!("Could not find entity for bone: {:?}", bone_name);
                        continue;
                    }
                };

                humanoid_bones.0.insert(bone_name, node_entity);
            }
        }
    }
}
