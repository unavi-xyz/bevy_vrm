use std::fmt::Display;

use gltf_kun::graph::{ByteNode, Graph, NodeIndex, OtherEdgeHelpers, Weight, gltf::Mesh};
use serde::{Deserialize, Serialize};
use serde_vrm::vrm0::FirstPersonFlag;

#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum MeshAnnotationEdges {
    #[serde(rename = "VRM/MeshAnnotation/Mesh")]
    Mesh,
}

impl Display for MeshAnnotationEdges {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = serde_json::to_string(self).unwrap();
        f.write_str(&string)?;
        Ok(())
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MeshAnnotationWeight {
    pub first_person_flag: FirstPersonFlag,
}

impl From<&Vec<u8>> for MeshAnnotationWeight {
    fn from(bytes: &Vec<u8>) -> Self {
        if bytes.is_empty() {
            return Self::default();
        }
        serde_json::from_slice(bytes).expect("Failed to deserialize weight")
    }
}

impl From<&MeshAnnotationWeight> for Vec<u8> {
    fn from(value: &MeshAnnotationWeight) -> Self {
        serde_json::to_vec(value).expect("Failed to serialize weight")
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MeshAnnotation(pub NodeIndex);

impl From<NodeIndex> for MeshAnnotation {
    fn from(index: NodeIndex) -> Self {
        Self(index)
    }
}

impl From<MeshAnnotation> for NodeIndex {
    fn from(annotation: MeshAnnotation) -> Self {
        annotation.0
    }
}

impl ByteNode<MeshAnnotationWeight> for MeshAnnotation {}
impl OtherEdgeHelpers for MeshAnnotation {}

impl MeshAnnotation {
    pub fn new(graph: &mut Graph) -> Self {
        let weight = &MeshAnnotationWeight::default();
        let node = graph.add_node(Weight::Bytes(weight.into()));
        Self(node)
    }

    pub fn mesh(&self, graph: &Graph) -> Option<Mesh> {
        self.find_property(graph, &MeshAnnotationEdges::Mesh.to_string())
    }
    pub fn set_mesh(&self, graph: &mut Graph, mesh: Option<Mesh>) {
        self.set_property(graph, MeshAnnotationEdges::Mesh.to_string(), mesh);
    }
}
