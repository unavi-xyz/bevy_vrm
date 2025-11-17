//! Bevy plugin for loading [VRM](https://vrm.dev/en/) avatars.
//! Aims to support both the VRM 0.0 and VRM 1.0 standards.

use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_gltf_kun::{GltfKunPlugin, import::gltf::scene::GltfScene};
use bevy_shader_mtoon::MtoonPlugin;
use loader::{Vrm, VrmLoader};
use serde_vrm::vrm0::FirstPersonFlag;

use crate::spring_bones::SpringBonePlugin;

#[cfg(feature = "animations")]
pub mod animations;
pub mod extensions;
pub mod first_person;
pub mod loader;
pub mod spring_bones;

pub mod mtoon {
    pub use bevy_shader_mtoon::*;
}

pub use serde_vrm::vrm0::BoneName;

pub struct VrmPlugin;
pub struct VrmPlugins;

impl PluginGroup for VrmPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(VrmPlugin)
            .add(SpringBonePlugin)
    }
}

impl Plugin for VrmPlugin {
    fn build(&self, app: &mut App) {
        // TODO: Dont use default GltfKunPlugin
        app.add_plugins((GltfKunPlugin::default(), MtoonPlugin))
            .init_asset::<Vrm>()
            .init_asset_loader::<VrmLoader>()
            .register_type::<BoneName>()
            .register_type::<FirstPersonFlag>()
            .add_observer(first_person::setup_first_person)
            .add_systems(Update, spawn_vrm_scenes);
    }
}

fn spawn_vrm_scenes(
    gltf_scenes: Res<Assets<GltfScene>>,
    mut commands: Commands,
    mut scene_spawner: ResMut<SceneSpawner>,
    to_spawn: Query<(Entity, &VrmInstance), Without<VrmSceneSpawned>>,
    vrms: Res<Assets<Vrm>>,
) {
    for (entity, vrm_handle) in to_spawn.iter() {
        let Some(vrm) = vrms.get(&vrm_handle.0) else {
            continue;
        };

        let vrm_scene_handle = match &vrm.gltf.default_scene {
            Some(handle) => handle,
            None => match vrm.gltf.scenes.first() {
                Some(handle) => handle,
                None => continue,
            },
        };

        let Some(gltf_scene) = gltf_scenes.get(vrm_scene_handle) else {
            continue;
        };

        // Spawn the scene as a child of this entity.
        scene_spawner.spawn_as_child(gltf_scene.scene.clone(), entity);

        // Mark as spawned.
        commands.entity(entity).insert(VrmSceneSpawned);
    }
}

#[derive(Component, Default)]
pub struct VrmInstance(pub Handle<Vrm>);

/// Marker component added after a VRM's scene has been spawned.
#[derive(Component)]
pub struct VrmSceneSpawned;
