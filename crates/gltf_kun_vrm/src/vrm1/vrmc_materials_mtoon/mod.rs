use gltf_kun::{extensions::Extension, graph::NodeIndex};

pub const EXTENSION_NAME: &str = "VRMC_materials_mtoon";

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct VrmcMaterialsMtoon(pub NodeIndex);

impl From<NodeIndex> for VrmcMaterialsMtoon {
    fn from(index: NodeIndex) -> Self {
        Self(index)
    }
}

impl From<VrmcMaterialsMtoon> for NodeIndex {
    fn from(mtoon: VrmcMaterialsMtoon) -> Self {
        mtoon.0
    }
}

impl Extension for VrmcMaterialsMtoon {
    fn name() -> &'static str {
        EXTENSION_NAME
    }
}
