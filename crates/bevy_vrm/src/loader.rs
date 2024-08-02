use std::fmt::Debug;

use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
    utils::HashMap,
};
use bevy_gltf_kun::import::gltf::{
    loader::{GltfError, GltfLoader},
    mesh::GltfMesh,
    GltfKun,
};
use gltf_kun::graph::{
    gltf::{GltfDocument, GltfWeight},
    ByteNode, Extensions, Weight,
};
use serde_vrm::vrm0::FirstPersonFlag;
use thiserror::Error;

use crate::extensions::VrmExtensions;

#[derive(Asset, TypePath, Debug)]
pub struct Vrm {
    pub gltf: GltfKun,
    pub mesh_annotations: HashMap<Handle<GltfMesh>, FirstPersonFlag>,
}

#[derive(Default)]
pub struct VrmLoader(pub GltfLoader<VrmExtensions>);

#[derive(Debug, Error)]
pub enum VrmError {
    #[error(transparent)]
    Gltf(#[from] GltfError),
}

impl AssetLoader for VrmLoader {
    type Asset = Vrm;
    type Settings = ();
    type Error = VrmError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        settings: &'a Self::Settings,
        load_context: &'a mut LoadContext,
    ) -> impl bevy::utils::ConditionalSendFuture<Output = Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let gltf = self.0.load(reader, settings, load_context).await?;

            let doc_idx = gltf
                .graph
                .node_indices()
                .find(|n| {
                    let weight = gltf.graph.node_weight(*n);
                    matches!(weight, Some(Weight::Gltf(GltfWeight::Document)))
                })
                .unwrap();
            let doc = GltfDocument(doc_idx);

            let ext = doc
                .get_extension::<gltf_kun_vrm::vrm0::Vrm>(&gltf.graph)
                .unwrap();

            let mut mesh_annotations = HashMap::default();

            for annotation in ext.mesh_annotations(&gltf.graph) {
                let meshes = doc.meshes(&gltf.graph);

                if let Some(mesh) = annotation.mesh(&gltf.graph) {
                    let mesh_idx = meshes.iter().position(|m| *m == mesh).unwrap();
                    let handle = gltf.meshes[mesh_idx].clone();

                    let annotation_data = annotation.read(&gltf.graph);
                    mesh_annotations.insert(handle, annotation_data.first_person_flag);
                }
            }

            Ok(Vrm {
                gltf,
                mesh_annotations,
            })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["vrm"]
    }
}
