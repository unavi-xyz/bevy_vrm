use std::fmt::Debug;

use bevy::{
    asset::{
        AssetLoader, LoadContext,
        io::{Reader, VecReader},
    },
    prelude::*,
};
use bevy_gltf_kun::import::gltf::{
    GltfKun,
    loader::{GlbLoader, GltfError, GltfLoader},
};
use thiserror::Error;

use crate::extensions::VrmExtensions;

#[derive(Asset, TypePath, Debug)]
pub struct Vrm {
    pub gltf: GltfKun,
}

#[derive(Default)]
pub struct VrmLoader {
    pub gltf_loader: GltfLoader<VrmExtensions>,
    pub glb_loader: GlbLoader<VrmExtensions>,
}

#[derive(Debug, Error)]
pub enum VrmError {
    #[error(transparent)]
    Gltf(#[from] GltfError),
}

impl AssetLoader for VrmLoader {
    type Asset = Vrm;
    type Settings = ();
    type Error = VrmError;

    fn load(
        &self,
        reader: &mut dyn Reader,
        settings: &Self::Settings,
        load_context: &mut LoadContext,
    ) -> impl bevy::tasks::ConditionalSendFuture<Output = std::result::Result<Self::Asset, Self::Error>>
    {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader
                .read_to_end(&mut bytes)
                .await
                .map_err(|e| VrmError::Gltf(GltfError::Io(e)))?;

            let is_glb = bytes.len() >= 4 && &bytes[0..4] == b"glTF";

            let gltf = if is_glb {
                let mut vec_reader = VecReader::new(bytes);
                self.glb_loader
                    .load(&mut vec_reader, settings, load_context)
                    .await?
            } else {
                let mut vec_reader = VecReader::new(bytes);
                self.gltf_loader
                    .load(&mut vec_reader, settings, load_context)
                    .await?
            };

            Ok(Vrm { gltf })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["vrm"]
    }
}
