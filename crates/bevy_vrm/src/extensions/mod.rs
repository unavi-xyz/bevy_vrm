use bevy::ecs::world::EntityWorldMut;
use bevy_gltf_kun::import::{extensions::BevyImportExtensions, gltf::document::ImportContext};
use gltf_kun::{
    extensions::{ExtensionExport, ExtensionImport},
    graph::{
        gltf::{GltfDocument, Node},
        Graph,
    },
    io::format::gltf::GltfFormat,
};

// pub mod vrm0;
// pub mod vrmc_vrm;

pub struct VrmExtensions;

impl ExtensionExport<GltfDocument, GltfFormat> for VrmExtensions {
    fn export(
        graph: &mut Graph,
        doc: &GltfDocument,
        format: &mut GltfFormat,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

impl ExtensionImport<GltfDocument, GltfFormat> for VrmExtensions {
    fn import(
        graph: &mut Graph,
        format: &mut GltfFormat,
        doc: &GltfDocument,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

impl BevyImportExtensions<GltfDocument> for VrmExtensions {
    fn import_node(context: &mut ImportContext, entity: &mut EntityWorldMut, node: Node) {}
}
