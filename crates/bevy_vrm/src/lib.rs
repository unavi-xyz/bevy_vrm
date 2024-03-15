use bevy::prelude::*;
use bevy_gltf_kun::import::gltf::{GltfAssetPlugin, GltfKun};
use bevy_shader_mtoon::MtoonPlugin;
use loader::{Vrm, VrmLoader};

pub mod extensions;
pub mod loader;

pub mod mtoon {
    pub use bevy_shader_mtoon::*;
}

pub struct VrmPlugin;

impl Plugin for VrmPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((GltfAssetPlugin, MtoonPlugin))
            .init_asset::<Vrm>()
            .init_asset_loader::<VrmLoader>();
    }
}

#[derive(Bundle, Default)]
pub struct VrmBundle {
    pub scene_bundle: SceneBundle,
    pub vrm: Handle<Vrm>,
}
