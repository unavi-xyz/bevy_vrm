use gltf_kun::{extensions::Extension, graph::NodeIndex};

pub mod weight;

pub const EXTENSION_NAME: &str = "VRM";

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Vrm(pub NodeIndex);

impl From<NodeIndex> for Vrm {
    fn from(index: NodeIndex) -> Self {
        Self(index)
    }
}

impl From<Vrm> for NodeIndex {
    fn from(physics_shape: Vrm) -> Self {
        physics_shape.0
    }
}

impl Extension for Vrm {
    fn name() -> &'static str {
        EXTENSION_NAME
    }
}
