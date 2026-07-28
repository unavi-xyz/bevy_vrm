//! Bevy plugin for loading [VRM](https://vrm.dev/en/) avatars.
//! Currently loads VRM 0.0 avatars; VRM 1.0 support is in progress.

use bevy::{
    gltf::{
        DefaultGltfImageSampler,
        GltfLoader,
        GltfSkinnedMeshBoundsPolicy,
        convert_coordinates::GltfConvertCoordinates,
        extensions::GltfExtensionHandlers,
    },
    image::{
        CompressedImageFormatSupport,
        CompressedImageFormats,
    },
    platform::collections::HashMap,
    prelude::*,
};
use bevy_shader_mtoon::MtoonPlugin;
use extensions::VrmHandler;
use loader::{
    Vrm,
    VrmLoader,
};
use serde_vrm::vrm0::FirstPersonFlag;

use crate::spring_bones::SpringBonePlugin;

#[cfg(feature = "animations")] pub mod animations;
pub mod extensions;
pub mod first_person;
pub mod loader;
pub mod spring_bones;

pub mod mtoon {
    pub use bevy_shader_mtoon::*;
}

pub use serde_vrm::vrm0::BoneName;

pub struct VrmPlugins;

impl PluginGroup for VrmPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        bevy::app::PluginGroupBuilder::start::<Self>()
            .add(VrmPlugin)
            .add(SpringBonePlugin)
    }
}

pub struct VrmPlugin;

impl Plugin for VrmPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MtoonPlugin)
            .init_asset::<Vrm>()
            .register_type::<BoneName>()
            .register_type::<FirstPersonFlag>()
            .add_observer(first_person::setup_first_person)
            .add_systems(Update, spawn_vrm_scenes);
    }

    fn finish(&self, app: &mut App) {
        {
            let handlers = app.world().resource::<GltfExtensionHandlers>().0.clone();
            #[cfg(not(target_family = "wasm"))]
            handlers
                .write_blocking()
                .push(Box::new(VrmHandler::default()));
            #[cfg(target_family = "wasm")]
            bevy::tasks::block_on(async {
                handlers.write().await.push(Box::new(VrmHandler::default()));
            });
        }

        let supported_compressed_formats = app
            .world()
            .get_resource::<CompressedImageFormatSupport>()
            .map_or(CompressedImageFormats::NONE, |r| r.0);
        let default_sampler = app
            .world()
            .resource::<DefaultGltfImageSampler>()
            .get_internal();
        let extensions = app.world().resource::<GltfExtensionHandlers>().0.clone();

        app.register_asset_loader(VrmLoader {
            gltf_loader: GltfLoader {
                supported_compressed_formats,
                custom_vertex_attributes: HashMap::default(),
                default_sampler,
                default_convert_coordinates: GltfConvertCoordinates::default(),
                extensions,
                default_skinned_mesh_bounds_policy: GltfSkinnedMeshBoundsPolicy::default(),
            },
        });
    }
}

fn spawn_vrm_scenes(
    mut commands: Commands,
    to_spawn: Query<(Entity, &VrmInstance), Without<VrmInstanceReady>>,
    vrms: Res<Assets<Vrm>>,
) {
    for (entity, vrm_handle) in to_spawn.iter() {
        let Some(vrm) = vrms.get(&vrm_handle.0) else {
            continue;
        };

        let scene = match &vrm.gltf.default_scene {
            Some(handle) => handle.clone(),
            None => match vrm.gltf.scenes.first() {
                Some(handle) => handle.clone(),
                None => continue,
            },
        };

        commands
            .entity(entity)
            .insert((WorldAssetRoot(scene), VrmInstanceReady));
    }
}

#[derive(Component, Default)]
pub struct VrmInstance(pub Handle<Vrm>);

/// Marks a [`VrmInstance`] whose scene has been spawned.
#[derive(Component)]
pub struct VrmInstanceReady;
