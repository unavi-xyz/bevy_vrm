//! `MToon` material construction from VRM 0.0 and VRM 1.0 data.

use bevy::{
    asset::LoadContext,
    gltf::GltfAssetLabel,
    image::Image,
    prelude::*,
};

pub mod vrm0;
pub mod vrm1;

fn texture_handle(load_context: &mut LoadContext, index: usize) -> Handle<Image> {
    load_context.get_label_handle(GltfAssetLabel::Texture(index).to_string())
}
