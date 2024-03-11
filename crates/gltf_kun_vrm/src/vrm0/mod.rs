use gltf_kun::{
    extensions::Extension,
    graph::{gltf::TextureInfo, ByteNode, Graph, NodeIndex, OtherEdgeHelpers},
};
use serde::{Deserialize, Serialize};

use self::{
    blend_shape_group::BlendShapeGroup, bone::Bone, mesh_annotation::MeshAnnotation,
    weight::VrmWeight,
};

pub mod bind;
pub mod blend_shape_group;
pub mod bone;
pub mod bone_group;
pub mod collider_group;
pub mod import;
pub mod mesh_annotation;
pub mod weight;

pub const EXTENSION_NAME: &str = "VRM";

#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum VrmEdge {
    #[serde(rename = "VRM/BlendShapeGroup")]
    BlendShapeGroup,
    #[serde(rename = "VRM/FirstPersonBone")]
    FirstPersonBone,
    #[serde(rename = "VRM/HumanBone")]
    HumanBone,
    #[serde(rename = "VRM/MeshAnnotation")]
    MeshAnnotation,
    #[serde(rename = "VRM/Thumbnail")]
    Thumbnail,
}

impl ToString for VrmEdge {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Vrm(pub NodeIndex);

impl From<NodeIndex> for Vrm {
    fn from(index: NodeIndex) -> Self {
        Self(index)
    }
}

impl From<Vrm> for NodeIndex {
    fn from(vrm: Vrm) -> Self {
        vrm.0
    }
}

impl ByteNode<VrmWeight> for Vrm {}
impl OtherEdgeHelpers for Vrm {}

impl Extension for Vrm {
    fn name() -> &'static str {
        EXTENSION_NAME
    }
}

impl Vrm {
    pub fn blend_shape_groups(&self, graph: &Graph) -> Vec<BlendShapeGroup> {
        self.find_properties(graph, &VrmEdge::BlendShapeGroup.to_string())
            .collect()
    }
    pub fn add_blend_shape_group(&self, graph: &mut Graph, group: BlendShapeGroup) {
        self.add_property(graph, VrmEdge::BlendShapeGroup.to_string(), group);
    }
    pub fn remove_blend_shape_group(&self, graph: &mut Graph, group: BlendShapeGroup) {
        self.remove_property(graph, &VrmEdge::BlendShapeGroup.to_string(), group);
    }

    pub fn first_person_bone(&self, graph: &Graph) -> Option<Bone> {
        self.find_property(graph, &VrmEdge::FirstPersonBone.to_string())
    }
    pub fn set_first_person_bone(&self, graph: &mut Graph, bone: Option<Bone>) {
        self.set_property(graph, VrmEdge::FirstPersonBone.to_string(), bone);
    }

    pub fn human_bones(&self, graph: &Graph) -> Vec<Bone> {
        self.find_properties(graph, &VrmEdge::HumanBone.to_string())
            .collect()
    }
    pub fn add_human_bone(&self, graph: &mut Graph, bone: Bone) {
        self.add_property(graph, VrmEdge::HumanBone.to_string(), bone);
    }
    pub fn remove_human_bone(&self, graph: &mut Graph, bone: Bone) {
        self.remove_property(graph, &VrmEdge::HumanBone.to_string(), bone);
    }

    pub fn mesh_annotations(&self, graph: &Graph) -> Vec<MeshAnnotation> {
        self.find_properties(graph, &VrmEdge::MeshAnnotation.to_string())
            .collect()
    }
    pub fn add_mesh_annotation(&self, graph: &mut Graph, annotation: MeshAnnotation) {
        self.add_property(graph, VrmEdge::MeshAnnotation.to_string(), annotation);
    }
    pub fn remove_mesh_annotation(&self, graph: &mut Graph, annotation: MeshAnnotation) {
        self.remove_property(graph, &VrmEdge::MeshAnnotation.to_string(), annotation);
    }

    pub fn thumbnail(&self, graph: &Graph) -> Option<TextureInfo> {
        self.find_property(graph, &VrmEdge::Thumbnail.to_string())
    }
    pub fn set_thumbnail(&self, graph: &mut Graph, texture: Option<TextureInfo>) {
        self.set_property(graph, VrmEdge::Thumbnail.to_string(), texture);
    }
}

#[cfg(test)]
mod tests {
    use gltf_kun::graph::GraphNodeWeight;

    use super::*;

    #[test]
    fn blend_shape_groups() {
        let mut graph = Graph::new();

        let vrm = Vrm::new(&mut graph);
        let group = BlendShapeGroup::new(&mut graph);

        vrm.add_blend_shape_group(&mut graph, group);
        assert_eq!(vrm.blend_shape_groups(&graph), vec![group]);

        let group_2 = BlendShapeGroup::new(&mut graph);
        vrm.add_blend_shape_group(&mut graph, group_2);
        assert_eq!(vrm.blend_shape_groups(&graph), vec![group, group_2]);

        vrm.remove_blend_shape_group(&mut graph, group);
        assert_eq!(vrm.blend_shape_groups(&graph), vec![group_2]);
    }

    #[test]
    fn first_person_bone() {
        let mut graph = Graph::new();

        let vrm = Vrm::new(&mut graph);
        let bone = Bone::new(&mut graph);

        vrm.set_first_person_bone(&mut graph, Some(bone));
        assert_eq!(vrm.first_person_bone(&graph), Some(bone));

        vrm.set_first_person_bone(&mut graph, None);
        assert_eq!(vrm.first_person_bone(&graph), None);
    }

    #[test]
    fn human_bones() {
        let mut graph = Graph::new();

        let vrm = Vrm::new(&mut graph);
        let bone = Bone::new(&mut graph);

        vrm.add_human_bone(&mut graph, bone);
        assert_eq!(vrm.human_bones(&graph), vec![bone]);

        let bone_2 = Bone::new(&mut graph);
        vrm.add_human_bone(&mut graph, bone_2);
        assert_eq!(vrm.human_bones(&graph), vec![bone, bone_2]);

        vrm.remove_human_bone(&mut graph, bone);
        assert_eq!(vrm.human_bones(&graph), vec![bone_2]);
    }

    #[test]
    fn mesh_annotations() {
        let mut graph = Graph::new();

        let vrm = Vrm::new(&mut graph);
        let annotation = MeshAnnotation::new(&mut graph);

        vrm.add_mesh_annotation(&mut graph, annotation);
        assert_eq!(vrm.mesh_annotations(&graph), vec![annotation]);

        let annotation_2 = MeshAnnotation::new(&mut graph);
        vrm.add_mesh_annotation(&mut graph, annotation_2);
        assert_eq!(vrm.mesh_annotations(&graph), vec![annotation, annotation_2]);

        vrm.remove_mesh_annotation(&mut graph, annotation);
        assert_eq!(vrm.mesh_annotations(&graph), vec![annotation_2]);
    }

    #[test]
    fn thumbnail() {
        let mut graph = Graph::new();

        let vrm = Vrm::new(&mut graph);
        let texture = TextureInfo::new(&mut graph);

        vrm.set_thumbnail(&mut graph, Some(texture));
        assert_eq!(vrm.thumbnail(&graph), Some(texture));

        vrm.set_thumbnail(&mut graph, None);
        assert_eq!(vrm.thumbnail(&graph), None);
    }
}
