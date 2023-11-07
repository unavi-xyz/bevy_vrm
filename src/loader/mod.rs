use std::fmt::Debug;

use bevy::{
    asset::{
        io::{Reader, VecReader},
        AssetLoader, AsyncReadExt, LoadContext,
    },
    gltf::{GltfError, GltfLoader},
    prelude::*,
    render::texture::CompressedImageFormats,
    utils::HashMap,
};
use goth_gltf::default_extensions;
use nanoserde::{DeJson, DeJsonErr};

use crate::Vrm;

mod vrm;
mod vrm0;

#[derive(Default)]
pub struct VrmLoader;

#[derive(Debug, thiserror::Error)]
pub enum VrmError {
    #[error("{0}")]
    DeJsonErr(#[from] DeJsonErr),
    #[error("{0}")]
    GltfError(#[from] GltfError),
    #[error("{0}")]
    IoError(#[from] std::io::Error),
}

impl AssetLoader for VrmLoader {
    type Asset = Vrm;
    type Settings = ();
    type Error = VrmError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        let gltf_loader = GltfLoader {
            custom_vertex_attributes: HashMap::default(),
            supported_compressed_formats: CompressedImageFormats::default(),
        };

        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            let gltf = gltf_loader
                .load(&mut VecReader::new(bytes.clone()), &(), load_context)
                .await;

            let gltf = match gltf {
                Ok(gltf) => gltf,
                Err(err) => {
                    return Err(VrmError::GltfError(err));
                }
            };

            load_vrm(gltf, &bytes, load_context).await
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

async fn load_vrm<'a, 'b>(
    gltf: bevy::gltf::Gltf,
    bytes: &'a [u8],
    load_context: &'a mut LoadContext<'b>,
) -> Result<Vrm, VrmError> {
    let (gltf_file, _) = goth_gltf::Gltf::from_bytes(&bytes)?;

    let mut vrm = Vrm {
        gltf,
        mtoon_materials: HashMap::default(),
        mtoon_markers: Vec::default(),
        mtoon_replaced: false,
    };

    if let Ok(_) = vrm0::load_gltf(&mut vrm, &gltf_file, load_context) {
        info!("VRM 0.0 loaded");
    } else if let Ok(_) = vrm::load_gltf(&mut vrm, &gltf_file, load_context) {
        info!("VRM 1.0 loaded");
    } else {
        error!("VRM extension not found");
    }

    Ok(vrm)
}
