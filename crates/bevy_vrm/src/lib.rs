use bevy::{prelude::*, utils::HashMap};
use bevy_gltf_kun::import::gltf::GltfAssetPlugin;
use bevy_shader_mtoon::MtoonPlugin;
use loader::{Vrm, VrmLoader};

mod auto_scene;
pub mod extensions;
mod humanoid_bones;
pub mod loader;

pub mod mtoon {
    pub use bevy_shader_mtoon::*;
}

pub use serde_vrm::vrm0::BoneName;

pub struct VrmPlugin;

impl Plugin for VrmPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((GltfAssetPlugin, MtoonPlugin))
            .init_asset::<Vrm>()
            .init_asset_loader::<VrmLoader>()
            .add_systems(
                Update,
                (
                    humanoid_bones::set_humanoid_bones,
                    auto_scene::set_vrm_scene,
                ),
            );
    }
}

#[derive(Bundle, Default)]
pub struct VrmBundle {
    pub auto_scene: AutoScene,
    pub humanoid_bones: HumanoidBones,
    pub scene_bundle: SceneBundle,
    pub vrm: Handle<Vrm>,
}

/// Automatically sets the scene to the loaded VRM's default scene.
#[derive(Component, Default)]
pub struct AutoScene;

#[derive(Component, Default)]
pub struct HumanoidBones(pub HashMap<BoneName, Entity>);
