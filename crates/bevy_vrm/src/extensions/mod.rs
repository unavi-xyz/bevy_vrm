use bevy::ecs::system::RunSystemOnce;
use bevy::prelude::*;
use bevy_gltf_kun::import::{extensions::BevyImportExtensions, gltf::document::ImportContext};
use gltf_kun::graph::gltf::GltfWeight;
use gltf_kun::graph::{ByteNode, Weight};
use gltf_kun::{
    extensions::ExtensionImport,
    graph::{
        gltf::{GltfDocument, Material, Node, Primitive, Scene},
        Extensions, Graph,
    },
    io::format::gltf::GltfFormat,
};
use gltf_kun_vrm::vrm0::Vrm;
use serde_vrm::vrm0::BoneName;

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
    fn import_scene(context: &mut ImportContext, _scene: Scene, world: &mut World) {
        let graph = &context.graph;

        let doc = match graph.node_indices().find(|n| {
            let weight = graph.node_weight(*n);
            matches!(weight, Some(Weight::Gltf(GltfWeight::Document)))
        }) {
            Some(doc) => GltfDocument(doc),
            None => {
                info!("failed to select gltf doc for vr0 loading");
                return;
            }
        };

        let ext = match doc.get_extension::<gltf_kun_vrm::vrm0::Vrm>(graph) {
            Some(ext) => ext,
            None => {
                info!("failed to select vrm 0 extension for vrm");
                return;
            }
        };

        for bone in ext.human_bones(graph) {
            let node = match bone.node(graph) {
                Some(n) => n,
                None => continue,
            };

            let weight = bone.read(graph);

            let bone_name = match weight.name {
                Some(b) => b,
                None => continue,
            };

            let node_handle = match context.gltf.node_handles.get(&node) {
                Some(handle) => handle.clone(),
                None => continue,
            };

            let node_name = context.gltf.named_nodes.iter().find_map(|(name, n)| {
                if *n == node_handle {
                    Some(name.clone())
                } else {
                    None
                }
            });

            let node_name = match node_name {
                Some(n) => n,
                None => continue,
            };

            world.run_system_once_with(
                (node_name, bone_name),
                |In((node_name, bone_name)): In<(String, BoneName)>,
                 mut commands: Commands,
                 names: Query<(Entity, &Name)>| {
                    let node_entity = match names.iter().find_map(|(entity, name)| {
                        if name.as_str() == node_name.as_str() {
                            print!("{}", name);
                            Some(entity)
                        } else {
                            None
                        }
                    }) {
                        Some(entity) => entity,
                        None => {
                            warn!("Could not find entity for bone: {:?}", bone_name);
                            return;
                        }
                    };
                    commands.entity(node_entity).insert(bone_name);
                },
            );
        }
    }
}
