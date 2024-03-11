use gltf_kun::{
    extensions::ExtensionExport,
    graph::{gltf::GltfDocument, Graph},
    io::format::gltf::GltfFormat,
};

use super::Vrm;

impl ExtensionExport<GltfDocument, GltfFormat> for Vrm {
    fn export(
        graph: &mut Graph,
        doc: &GltfDocument,
        format: &mut GltfFormat,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
