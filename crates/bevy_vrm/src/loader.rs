use bevy::{
    asset::{
        AssetLoader,
        LoadContext,
        io::Reader,
    },
    gltf::{
        Gltf,
        GltfError,
        GltfLoader,
        GltfLoaderSettings,
    },
    prelude::*,
};
use thiserror::Error;

#[derive(Asset, TypePath, Debug)]
pub struct Vrm {
    pub gltf: Gltf,
}

#[derive(TypePath)]
pub struct VrmLoader {
    pub gltf_loader: GltfLoader,
}

#[derive(Debug, Error)]
pub enum VrmError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Gltf(#[from] GltfError),
}

impl AssetLoader for VrmLoader {
    type Asset = Vrm;
    type Settings = GltfLoaderSettings;
    type Error = VrmError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        settings: &Self::Settings,
        load_context: &mut LoadContext<'_>,
    ) -> Result<Vrm, VrmError> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let gltf = GltfLoader::load_gltf(&self.gltf_loader, &bytes, load_context, settings).await?;

        Ok(Vrm { gltf })
    }

    fn extensions(&self) -> &[&str] {
        &["vrm"]
    }
}
