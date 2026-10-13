//! Bevy plugin for loading [VRM](https://vrm.dev/en/) avatars.
//! Loads VRM 0.0 and 1.0 avatars into a single 1.0-shaped component model.
//!
//! Supports humanoid bones, MToon materials, first person and spring
//! bones.
//! Not yet supported: expressions, look-at, node constraints and spring
//! colliders.

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
    mesh::MeshCompressionArgs,
    platform::collections::HashMap,
    prelude::*,
};
use bevy_shader_mtoon::MtoonPlugin;
use extensions::VrmHandler;
use instance::spawn_vrm_scenes;
use loader::{
    Vrm,
    VrmLoader,
};
use serde_vrm::vrm1::vrmc_vrm::{
    first_person::FirstPersonType,
    humanoid::HumanBoneName,
};

use crate::spring_bones::SpringBonePlugin;

#[cfg(feature = "animations")] pub mod animations;
pub mod extensions;
pub mod first_person;
pub mod instance;
pub mod loader;
pub mod spring_bones;

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
            .register_type::<HumanBoneName>()
            .register_type::<FirstPersonType>()
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
                // The MToon outline reads uncompressed `Float32x3` normals.
                default_mesh_compression: MeshCompressionArgs::none(),
            },
        });
    }
}
