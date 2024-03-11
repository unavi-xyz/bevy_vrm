use gltf_kun::graph::{gltf::Mesh, ByteNode, Graph, NodeIndex, OtherEdgeHelpers};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum MeshAnnotationEdges {
    #[serde(rename = "VRM/MeshAnnotation/Mesh")]
    Mesh,
}

impl ToString for MeshAnnotationEdges {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MeshAnnotationWeight {
    pub first_person_flag: Option<String>,
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
    pub fn mesh(&self, graph: &Graph) -> Option<Mesh> {
        self.find_property(graph, &MeshAnnotationEdges::Mesh.to_string())
    }
    pub fn set_mesh(&self, graph: &mut Graph, mesh: Option<Mesh>) {
        self.set_property(graph, MeshAnnotationEdges::Mesh.to_string(), mesh);
    }
}
