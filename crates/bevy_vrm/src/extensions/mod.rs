use bevy::ecs::world::EntityWorldMut;
use bevy_gltf_kun::import::{extensions::BevyImportExtensions, gltf::document::ImportContext};
use gltf_kun::{
    extensions::ExtensionImport,
    graph::{
        gltf::{GltfDocument, Node, Primitive},
        Extensions, Graph,
    },
    io::format::gltf::GltfFormat,
};
use gltf_kun_vrm::vrm0::Vrm;

use self::vrm0::import_primitive_material;

pub mod vrm0;
pub mod vrm1;

pub struct VrmExtensions;

impl ExtensionImport<GltfDocument, GltfFormat> for VrmExtensions {
    fn import(
        graph: &mut Graph,
        format: &mut GltfFormat,
        doc: &GltfDocument,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Vrm::import(graph, format, doc)?;

        Ok(())
    }
}

impl BevyImportExtensions<GltfDocument> for VrmExtensions {
    fn import_node(_context: &mut ImportContext, _entity: &mut EntityWorldMut, _node: Node) {}

    fn import_primitive(
        context: &mut ImportContext,
        entity: &mut EntityWorldMut,
        primitive: Primitive,
    ) {
        if let Some(ext) = primitive.get_extension::<Vrm>(context.graph) {
            import_primitive_material(context, entity, ext, primitive);
        }
    }

    fn import_root(_context: &mut ImportContext) {}
}
