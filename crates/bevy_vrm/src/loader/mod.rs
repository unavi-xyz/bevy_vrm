use std::fmt::Debug;

use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
    utils::BoxedFuture,
};
use bevy_gltf_kun::import::gltf::{
    loader::{GltfError, GltfLoader},
    GltfKun,
};
use thiserror::Error;

use crate::extensions::VrmExtensions;

use self::humanoid_bones::{load_humanoid_bones, HumanoidBones};

mod humanoid_bones;

#[derive(Asset, TypePath, Debug)]
pub struct Vrm {
    pub gltf: GltfKun,
    pub humanoid_bones: HumanoidBones,
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
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let gltf = self.0.load(reader, settings, load_context).await?;
            let humanoid_bones = load_humanoid_bones(&gltf).unwrap_or_default();

            Ok(Vrm {
                gltf,
                humanoid_bones,
            })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["vrm"]
    }
}
