use bevy::{prelude::*, utils::HashMap};
use bevy_gltf_kun::import::gltf::{scene::GltfScene, GltfAssetPlugin};
use bevy_shader_mtoon::MtoonPlugin;
use humanoid_bones::set_humanoid_bones;
use loader::{Vrm, VrmLoader};

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
            .add_systems(Update, (set_humanoid_bones, set_vrm_scene));
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

fn set_vrm_scene(
    gltf_scenes: Res<Assets<GltfScene>>,
    mut commands: Commands,
    scenes: Query<(Entity, &mut Handle<Scene>, &Handle<Vrm>), With<AutoScene>>,
    vrm: Res<Assets<Vrm>>,
) {
    for (entity, scene_handle, vrm_handle) in scenes.iter() {
        let vrm = match vrm.get(vrm_handle) {
            Some(vrm) => vrm,
            None => continue,
        };

        let vrm_scene = match &vrm.gltf.default_scene {
            Some(handle) => handle,
            None => match vrm.gltf.scenes.first() {
                Some(handle) => handle,
                None => continue,
            },
        };

        let vrm_scene = match gltf_scenes.get(vrm_scene) {
            Some(scene) => &scene.scene,
            None => continue,
        };

        if scene_handle.id() == vrm_scene.id() {
            continue;
        }

        commands.entity(entity).insert(vrm_scene.clone());
    }
}

#[derive(Component, Default)]
pub struct HumanoidBones(pub HashMap<BoneName, Entity>);
