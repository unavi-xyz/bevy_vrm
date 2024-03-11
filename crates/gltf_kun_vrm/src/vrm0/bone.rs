use gltf_kun::graph::{gltf::Node, ByteNode, Graph, NodeIndex, OtherEdgeHelpers, Weight};
use serde::{Deserialize, Serialize};
use serde_vrm::vrm0::BoneName;

#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum BoneEdges {
    #[serde(rename = "VRM/Bone/Node")]
    Node,
}

impl ToString for BoneEdges {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct BoneWeight {
    pub name: Option<BoneName>,
    pub use_default_values: Option<bool>,
}

impl From<&Vec<u8>> for BoneWeight {
    fn from(bytes: &Vec<u8>) -> Self {
        if bytes.is_empty() {
            return Self::default();
        }
        serde_json::from_slice(bytes).expect("Failed to deserialize weight")
    }
}

impl From<&BoneWeight> for Vec<u8> {
    fn from(value: &BoneWeight) -> Self {
        serde_json::to_vec(value).expect("Failed to serialize weight")
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Bone(pub NodeIndex);

impl From<NodeIndex> for Bone {
    fn from(index: NodeIndex) -> Self {
        Self(index)
    }
}

impl From<Bone> for NodeIndex {
    fn from(bone: Bone) -> Self {
        bone.0
    }
}

impl ByteNode<BoneWeight> for Bone {}
impl OtherEdgeHelpers for Bone {}

impl Bone {
    pub fn new(graph: &mut Graph) -> Self {
        let weight = &BoneWeight::default();
        let node = graph.add_node(Weight::Bytes(weight.into()));
        Self(node)
    }

    pub fn node(&self, graph: &Graph) -> Option<Node> {
        self.find_property(graph, &BoneEdges::Node.to_string())
    }
    pub fn set_node(&self, graph: &mut Graph, node: Option<Node>) {
        self.set_property(graph, BoneEdges::Node.to_string(), node);
    }
}
