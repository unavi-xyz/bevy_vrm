use bevy::prelude::*;
use bevy_gltf_kun::import::gltf::scene::GltfScene;

use crate::{VrmInstance, VrmScene, loader::Vrm};

/// Automatically sets the scene to the loaded VRM's default scene,
/// or the first scene if there is no default.
#[derive(Component, Default)]
pub struct AutoScene;

pub fn set_vrm_scene(
    gltf_scenes: Res<Assets<GltfScene>>,
    mut commands: Commands,
    scenes: Query<(Entity, &VrmScene, &VrmInstance), With<AutoScene>>,
    vrm: Res<Assets<Vrm>>,
) {
    for (entity, scene_handle, vrm_handle) in scenes.iter() {
        let vrm = match vrm.get(vrm_handle.0.id()) {
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

        if scene_handle.0.id() == vrm_scene.id() {
            continue;
        }

        commands.entity(entity).insert((
            VrmScene(vrm_scene.clone()),
            SceneRoot(vrm_scene.clone()),
        ));
    }
}
