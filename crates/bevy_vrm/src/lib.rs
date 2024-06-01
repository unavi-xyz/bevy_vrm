//! [Bevy](https://bevyengine.org/) plugin for loading [VRM](https://vrm.dev/en/) avatars.
//! Aims to support both the VRM 0.0 and VRM 1.0 standards.

use bevy::{prelude::*};
use bevy_gltf_kun::import::gltf::GltfAssetPlugin;
use bevy_shader_mtoon::MtoonPlugin;
use loader::{Vrm, VrmLoader};

mod auto_scene;
pub mod extensions;
pub mod loader;
mod spring_bones;

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
            .register_type::<BoneName>()
            .add_systems(
                Update,
                (auto_scene::set_vrm_scene, spring_bones::set_spring_bones),
            );
    }
}

#[derive(Bundle, Default)]
pub struct VrmBundle {
    pub auto_scene: AutoScene,
    pub scene_bundle: SceneBundle,
    pub spring_bones: SpringBones,
    pub vrm: Handle<Vrm>,
}

/// Automatically sets the scene to the loaded VRM's default scene.
#[derive(Component, Default)]
pub struct AutoScene;

#[derive(Component, Default)]
pub struct SpringBones(pub Vec<SpringBone>);

pub struct SpringBone {
    pub bones: Vec<Entity>,
    pub center: f32,
    pub drag_force: f32,
    pub gravity_dir: Vec3,
    pub gravity_power: f32,
    pub hit_radius: f32,
    pub stiffness: f32,
}
