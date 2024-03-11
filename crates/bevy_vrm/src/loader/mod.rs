use std::fmt::Debug;

use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
    utils::BoxedFuture,
};
use bevy_gltf_kun::import::gltf::loader::{GltfError, GltfLoader};
use thiserror::Error;

use crate::{extensions::VrmExtensions, Vrm};

// mod vrm;
// mod vrm0;

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
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let doc = self.0.load(reader, settings, load_context).await?;

            todo!()
        })
    }

    fn extensions(&self) -> &[&str] {
        &["vrm"]
    }
}

// #[derive(Default, Debug, Clone, DeJson)]
// pub struct RootExtensions {
//     #[nserde(rename = "VRM")]
//     pub vrm0: Option<super::extensions::vrm0::Vrm>,
//     #[nserde(rename = "VRMC_vrm")]
//     pub vrmc_vrm: Option<super::extensions::vrmc_vrm::VrmcVrm>,
// }

// #[derive(Debug, Default, Clone, Copy, DeJson)]
// pub struct Extensions;
//
// async fn load_vrm<'a, 'b>(
//     gltf: bevy::gltf::Gltf,
//     bytes: &'a [u8],
//     load_context: &'a mut LoadContext<'b>,
// ) -> Result<Vrm, VrmError> {
//     let doc = GlbIO::import_slice(bytes)?;
//
//     let mut vrm = Vrm {
//         gltf,
//         mtoon_materials: default(),
//         mtoon_markers: default(),
//         extensions: default(),
//     };
//
//     if let Ok(()) = vrm0::load_gltf(&mut vrm, &gltf_file, load_context) {
//         info!("VRM 0.0 loaded");
//     } else if let Ok(()) = vrm::load_gltf(&mut vrm, &gltf_file, load_context) {
//         info!("VRM 1.0 loaded");
//     } else {
//         error!("VRM extension not found");
//     };
//
//     Ok(vrm)
// }
