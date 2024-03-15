use bevy::{prelude::*, utils::HashMap};
use bevy_gltf_kun::import::gltf::node::GltfNode;
use gltf_kun::graph::{
    gltf::{GltfDocument, GltfWeight},
    ByteNode, Extensions, Weight,
};
use gltf_kun_vrm::vrm0::Vrm;
use serde_vrm::vrm0::BoneName;

use crate::GltfKun;

pub type HumanoidBones = HashMap<BoneName, Handle<GltfNode>>;

pub fn load_humanoid_bones(gltf: &GltfKun) -> Option<HumanoidBones> {
    let doc = match gltf.graph.node_indices().find(|n| {
        let weight = gltf.graph.node_weight(*n);
        matches!(weight, Some(Weight::Gltf(GltfWeight::Document)))
    }) {
        Some(doc) => GltfDocument(doc),
        None => return None,
    };

    let ext = match doc.get_extension::<Vrm>(&gltf.graph) {
        Some(ext) => ext,
        None => return None,
    };

    let mut humanoid_bones = HashMap::default();

    for bone in ext.human_bones(&gltf.graph) {
        let node = match bone.node(&gltf.graph) {
            Some(node) => node,
            None => continue,
        };

        let weight = bone.read(&gltf.graph);

        let name = match weight.name {
            Some(name) => name,
            None => continue,
        };

        let node_handle = match gltf.node_handles.get(&node) {
            Some(handle) => handle.clone(),
            None => continue,
        };

        humanoid_bones.insert(name, node_handle);
    }

    Some(humanoid_bones)
}
