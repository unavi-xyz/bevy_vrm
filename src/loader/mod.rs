use std::fmt::Debug;

use bevy::{
    asset::AssetLoader, gltf::GltfLoader, prelude::*, render::texture::CompressedImageFormats,
    utils::HashMap,
};
use goth_gltf::default_extensions;
use nanoserde::{DeJson, DeJsonErr};

mod vrm;
mod vrm0;

#[derive(Default)]
pub struct VrmLoader;

impl AssetLoader for VrmLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        let gltf_loader = GltfLoader {
            custom_vertex_attributes: HashMap::default(),
            supported_compressed_formats: CompressedImageFormats::default(),
        };

        Box::pin(async move {
            gltf_loader.load(bytes, load_context).await?;
            load_vrm_extensions(bytes).await?;
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["vrm"]
    }
}

#[derive(Default, Debug, Clone, DeJson)]
pub struct RootExtensions {
    #[nserde(rename = "VRM")]
    pub vrm0: Option<super::extensions::vrm0::Vrm>,
    #[nserde(rename = "VRMC_vrm")]
    pub vrmc_vrm: Option<super::extensions::vrmc_vrm::VrmcVrm>,
}

#[derive(Debug, Default, Clone, Copy, DeJson)]
pub struct Extensions;

impl goth_gltf::Extensions for Extensions {
    type RootExtensions = RootExtensions;
    type TextureExtensions = default_extensions::TextureExtensions;
    type TextureInfoExtensions = default_extensions::TextureInfoExtensions;
    type MaterialExtensions = default_extensions::MaterialExtensions<Self>;
    type BufferExtensions = default_extensions::BufferExtensions;
    type NodeExtensions = default_extensions::NodeExtensions;
    type NodeExtras = default_extensions::NodeExtras;
    type BufferViewExtensions = default_extensions::BufferViewExtensions;
}

async fn load_vrm_extensions(bytes: &[u8]) -> Result<(), DeJsonErr> {
    let (gltf, _) = goth_gltf::Gltf::from_bytes(&bytes)?;

    if let Ok(_) = vrm0::load_gltf(&gltf) {
        info!("VRM 0.0 loaded");
    } else if let Ok(_) = vrm::load_gltf(&gltf) {
        info!("VRM 1.0 loaded");
    } else {
        error!("VRM extension not found");
    }

    Ok(())
}
