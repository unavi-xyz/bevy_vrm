use bevy::{
    asset::AssetLoader, gltf::GltfLoader, render::texture::CompressedImageFormats, utils::HashMap,
};

#[derive(Default)]
pub struct VRMLoader;

impl AssetLoader for VRMLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        let gltf_loader = GltfLoader {
            custom_vertex_attributes: HashMap::default(),
            supported_compressed_formats: CompressedImageFormats::default(),
        };

        Box::pin(async move { Ok(gltf_loader.load(bytes, load_context).await?) })
    }

    fn extensions(&self) -> &[&str] {
        &["vrm"]
    }
}
