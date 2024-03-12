use gltf_kun::graph::{gltf::TextureInfo, ByteNode, Graph, NodeIndex, OtherEdgeHelpers, Weight};
use serde::{Deserialize, Serialize};
use serde_vrm::vrm0::{FloatProperties, KeywordMap, Shader, TagMap, VectorProperties};

#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum MaterialPropertyEdges {
    #[serde(rename = "VRM/MaterialProperty/MainTexture")]
    MainTexture,
    #[serde(rename = "VRM/MaterialProperty/ShadeTexture")]
    ShadeTexture,
    #[serde(rename = "VRM/MaterialProperty/BumpMap")]
    BumpMap,
    #[serde(rename = "VRM/MaterialProperty/SphereAdd")]
    SphereAdd,
    #[serde(rename = "VRM/MaterialProperty/EmissionMap")]
    EmissionMap,
}

impl ToString for MaterialPropertyEdges {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MaterialPropertyWeight {
    pub name: Option<String>,
    pub render_queue: Option<i32>,
    pub shader: Option<Shader>,
    pub float: Option<FloatProperties>,
    pub vector: Option<VectorProperties>,
    pub keyword_map: Option<KeywordMap>,
    pub tag_map: Option<TagMap>,
}

impl From<&Vec<u8>> for MaterialPropertyWeight {
    fn from(bytes: &Vec<u8>) -> Self {
        if bytes.is_empty() {
            return Self::default();
        }
        serde_json::from_slice(bytes).expect("Failed to deserialize weight")
    }
}

impl From<&MaterialPropertyWeight> for Vec<u8> {
    fn from(value: &MaterialPropertyWeight) -> Self {
        serde_json::to_vec(value).expect("Failed to serialize weight")
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MaterialProperty(pub NodeIndex);

impl From<NodeIndex> for MaterialProperty {
    fn from(index: NodeIndex) -> Self {
        Self(index)
    }
}

impl From<MaterialProperty> for NodeIndex {
    fn from(material: MaterialProperty) -> Self {
        material.0
    }
}

impl ByteNode<MaterialPropertyWeight> for MaterialProperty {}
impl OtherEdgeHelpers for MaterialProperty {}

impl MaterialProperty {
    pub fn new(graph: &mut Graph) -> Self {
        let weight = &MaterialPropertyWeight::default();
        let node = graph.add_node(Weight::Bytes(weight.into()));
        Self(node)
    }

    pub fn main_texture(&self, graph: &Graph) -> Option<TextureInfo> {
        self.find_property(graph, &MaterialPropertyEdges::MainTexture.to_string())
    }
    pub fn set_main_texture(&self, graph: &mut Graph, texture: Option<TextureInfo>) {
        self.set_property(
            graph,
            MaterialPropertyEdges::MainTexture.to_string(),
            texture,
        );
    }

    pub fn shade_texture(&self, graph: &Graph) -> Option<TextureInfo> {
        self.find_property(graph, &MaterialPropertyEdges::ShadeTexture.to_string())
    }
    pub fn set_shade_texture(&self, graph: &mut Graph, texture: Option<TextureInfo>) {
        self.set_property(
            graph,
            MaterialPropertyEdges::ShadeTexture.to_string(),
            texture,
        );
    }

    pub fn bump_map(&self, graph: &Graph) -> Option<TextureInfo> {
        self.find_property(graph, &MaterialPropertyEdges::BumpMap.to_string())
    }
    pub fn set_bump_map(&self, graph: &mut Graph, texture: Option<TextureInfo>) {
        self.set_property(graph, MaterialPropertyEdges::BumpMap.to_string(), texture);
    }

    pub fn sphere_add(&self, graph: &Graph) -> Option<TextureInfo> {
        self.find_property(graph, &MaterialPropertyEdges::SphereAdd.to_string())
    }
    pub fn set_sphere_add_texture(&self, graph: &mut Graph, texture: Option<TextureInfo>) {
        self.set_property(graph, MaterialPropertyEdges::SphereAdd.to_string(), texture);
    }

    pub fn emission_map(&self, graph: &Graph) -> Option<TextureInfo> {
        self.find_property(graph, &MaterialPropertyEdges::EmissionMap.to_string())
    }
    pub fn set_emission_map(&self, graph: &mut Graph, texture: Option<TextureInfo>) {
        self.set_property(
            graph,
            MaterialPropertyEdges::EmissionMap.to_string(),
            texture,
        );
    }
}
