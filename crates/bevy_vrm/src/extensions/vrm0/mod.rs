use bevy_gltf_kun::import::{extensions::RootExtensionImport, gltf::document::ImportContext};
use gltf_kun::{
    extensions::Extension,
    graph::{gltf::GltfDocument, Graph, NodeIndex},
};
use gltf_kun_vrm::vrm0::Vrm;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BevyVrm(pub Vrm);

impl From<NodeIndex> for BevyVrm {
    fn from(node: NodeIndex) -> Self {
        BevyVrm(Vrm::from(node))
    }
}

impl From<BevyVrm> for NodeIndex {
    fn from(vrm: BevyVrm) -> Self {
        vrm.0.into()
    }
}

impl Extension for BevyVrm {
    fn name() -> &'static str {
        Vrm::name()
    }

    fn new(graph: &mut Graph) -> Self {
        BevyVrm(Vrm::new(graph))
    }
}

impl RootExtensionImport<GltfDocument> for BevyVrm {
    fn import_root(context: &mut ImportContext, ext: Self) {
        // TODO: Implement
    }
}
