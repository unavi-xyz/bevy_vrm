use gltf_kun::graph::{gltf::Node, ByteNode, Graph, NodeIndex, OtherEdgeHelpers};
use serde::{Deserialize, Serialize};
use serde_vrm::vrm0::Vec3;

#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum ColliderGroupEdges {
    #[serde(rename = "VRM/ColliderGroup/Node")]
    Node,
}

impl ToString for ColliderGroupEdges {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ColliderGroupWeight {
    pub colliders: Vec<Collider>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Collider {
    pub offset: Vec3,
    pub radius: Option<f32>,
}

impl From<&Vec<u8>> for ColliderGroupWeight {
    fn from(bytes: &Vec<u8>) -> Self {
        if bytes.is_empty() {
            return Self::default();
        }
        serde_json::from_slice(bytes).expect("Failed to deserialize weight")
    }
}

impl From<&ColliderGroupWeight> for Vec<u8> {
    fn from(value: &ColliderGroupWeight) -> Self {
        serde_json::to_vec(value).expect("Failed to serialize weight")
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ColliderGroup(pub NodeIndex);

impl From<NodeIndex> for ColliderGroup {
    fn from(index: NodeIndex) -> Self {
        Self(index)
    }
}

impl From<ColliderGroup> for NodeIndex {
    fn from(group: ColliderGroup) -> Self {
        group.0
    }
}

impl ByteNode<ColliderGroupWeight> for ColliderGroup {}
impl OtherEdgeHelpers for ColliderGroup {}

impl ColliderGroup {
    pub fn node(&self, graph: &Graph) -> Option<Node> {
        self.find_property(graph, &ColliderGroupEdges::Node.to_string())
    }
    pub fn set_node(&self, graph: &mut Graph, node: Option<Node>) {
        self.set_property(graph, ColliderGroupEdges::Node.to_string(), node);
    }
}
