use gltf_kun::graph::{ByteNode, Graph, NodeIndex, OtherEdgeHelpers, Weight};
use serde::{Deserialize, Serialize};
use serde_vrm::vrm0::{MaterialBind, PresetName};

use super::bind::Bind;

#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum BlendShapeGroupEdges {
    #[serde(rename = "VRM/BlendShapeGroup/Bind")]
    Bind,
}

impl ToString for BlendShapeGroupEdges {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct BlendShapeGroupWeight {
    pub is_binary: Option<bool>,
    pub material_values: Vec<MaterialBind>,
    pub name: Option<String>,
    pub preset_name: Option<PresetName>,
}

impl From<&Vec<u8>> for BlendShapeGroupWeight {
    fn from(bytes: &Vec<u8>) -> Self {
        if bytes.is_empty() {
            return Self::default();
        }
        serde_json::from_slice(bytes).expect("Failed to deserialize weight")
    }
}

impl From<&BlendShapeGroupWeight> for Vec<u8> {
    fn from(value: &BlendShapeGroupWeight) -> Self {
        serde_json::to_vec(value).expect("Failed to serialize weight")
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BlendShapeGroup(pub NodeIndex);

impl From<NodeIndex> for BlendShapeGroup {
    fn from(index: NodeIndex) -> Self {
        Self(index)
    }
}

impl From<BlendShapeGroup> for NodeIndex {
    fn from(group: BlendShapeGroup) -> Self {
        group.0
    }
}

impl ByteNode<BlendShapeGroupWeight> for BlendShapeGroup {}
impl OtherEdgeHelpers for BlendShapeGroup {}

impl BlendShapeGroup {
    pub fn new(graph: &mut Graph) -> Self {
        let weight = &BlendShapeGroupWeight::default();
        let node = graph.add_node(Weight::Bytes(weight.into()));
        Self(node)
    }

    pub fn binds(&self, graph: &Graph) -> Vec<Bind> {
        self.find_properties(graph, &BlendShapeGroupEdges::Bind.to_string())
    }
    pub fn add_bind(&self, graph: &mut Graph, bind: Bind) {
        self.add_property(graph, BlendShapeGroupEdges::Bind.to_string(), bind);
    }
    pub fn remove_bind(&self, graph: &mut Graph, bind: Bind) {
        self.remove_property(graph, &BlendShapeGroupEdges::Bind.to_string(), bind);
    }
}
