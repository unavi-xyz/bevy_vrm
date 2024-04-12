use bevy::prelude::*;
use bevy_gltf_kun::import::{extensions::BevyImportExtensions, gltf::document::ImportContext};
use gltf_kun::{
    extensions::ExtensionImport,
    graph::{
        gltf::{GltfDocument, Material, Node, Primitive, Scene},
        Extensions, Graph,
    },
    io::format::gltf::GltfFormat,
};
use gltf_kun_vrm::vrm0::Vrm;

use self::vrm0::{import_material, import_primitive_material};

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
    fn import_material(
        context: &mut ImportContext,
        _standard_material: &mut StandardMaterial,
        material: Material,
    ) {
        if let Some(ext) = context.doc.get_extension::<Vrm>(context.graph) {
            import_material(context, material, ext);
        }
    }

    fn import_node(_context: &mut ImportContext, _entity: &mut EntityWorldMut, _node: Node) {}

    fn import_primitive(
        context: &mut ImportContext,
        entity: &mut EntityWorldMut,
        primitive: Primitive,
    ) {
        if let Some(ext) = context.doc.get_extension::<Vrm>(context.graph) {
            import_primitive_material(context, entity, ext, primitive);
        }
    }

    fn import_root(_context: &mut ImportContext) {}
    fn import_scene(_context: &mut ImportContext, _scene: Scene, _world: &mut World) {}
}
