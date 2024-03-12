use gltf_kun::graph::{ByteNode, Graph, NodeIndex, OtherEdgeHelpers, Weight};
use serde::{Deserialize, Serialize};
use serde_vrm::vrm0::Vec3;

use super::{bone::Bone, collider_group::ColliderGroup};

#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum BoneGroupEdges {
    #[serde(rename = "VRM/BoneGroup/Bone")]
    Bone,
    #[serde(rename = "VRM/BoneGroup/ColliderGroup")]
    ColliderGroup,
}

impl ToString for BoneGroupEdges {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct BoneGroupWeight {
    pub comment: Option<String>,
    pub stiffiness: Option<f32>,
    pub gravity_power: Option<f32>,
    pub gravity_dir: Vec3,
    pub drag_force: Option<f32>,
    pub center: Option<f32>,
    pub hit_radius: Option<f32>,
}

impl From<&Vec<u8>> for BoneGroupWeight {
    fn from(bytes: &Vec<u8>) -> Self {
        if bytes.is_empty() {
            return Self::default();
        }
        serde_json::from_slice(bytes).expect("Failed to deserialize weight")
    }
}

impl From<&BoneGroupWeight> for Vec<u8> {
    fn from(value: &BoneGroupWeight) -> Self {
        serde_json::to_vec(value).expect("Failed to serialize weight")
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BoneGroup(pub NodeIndex);

impl From<NodeIndex> for BoneGroup {
    fn from(index: NodeIndex) -> Self {
        Self(index)
    }
}

impl From<BoneGroup> for NodeIndex {
    fn from(group: BoneGroup) -> Self {
        group.0
    }
}

impl ByteNode<BoneGroupWeight> for BoneGroup {}
impl OtherEdgeHelpers for BoneGroup {}

impl BoneGroup {
    pub fn new(graph: &mut Graph) -> Self {
        let weight = &BoneGroupWeight::default();
        Self(graph.add_node(Weight::Bytes(weight.into())))
    }

    pub fn bones(&self, graph: &Graph) -> Vec<Bone> {
        self.find_properties(graph, &BoneGroupEdges::Bone.to_string())
    }
    pub fn add_bone(&self, graph: &mut Graph, bone: Bone) {
        self.add_property(graph, BoneGroupEdges::Bone.to_string(), bone);
    }
    pub fn remove_bone(&self, graph: &mut Graph, bone: Bone) {
        self.remove_property(graph, &BoneGroupEdges::Bone.to_string(), bone);
    }

    pub fn collider_groups(&self, graph: &Graph) -> Vec<ColliderGroup> {
        self.find_properties(graph, &BoneGroupEdges::ColliderGroup.to_string())
    }
    pub fn add_collider_group(&self, graph: &mut Graph, group: ColliderGroup) {
        self.add_property(graph, BoneGroupEdges::ColliderGroup.to_string(), group);
    }
    pub fn remove_collider_group(&self, graph: &mut Graph, group: ColliderGroup) {
        self.remove_property(graph, &BoneGroupEdges::ColliderGroup.to_string(), group);
    }
}
