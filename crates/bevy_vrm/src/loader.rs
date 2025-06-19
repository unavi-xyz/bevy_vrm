use std::fmt::Debug;

use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
};
use bevy_gltf_kun::import::gltf::{
    GltfKun,
    loader::{GltfError, GltfLoader},
};
use thiserror::Error;

use crate::extensions::VrmExtensions;

#[derive(Asset, TypePath, Debug)]
pub struct Vrm {
    pub gltf: GltfKun,
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
            Ok(Vrm { gltf })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["vrm"]
    }
}
