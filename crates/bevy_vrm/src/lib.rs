//! [Bevy](https://bevyengine.org/) plugin for loading [VRM](https://vrm.dev/en/) avatars.
//! Aims to support both the VRM 0.0 and VRM 1.0 standards.

use auto_scene::AutoScene;
use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_gltf_kun::GltfKunPlugin;
use bevy_shader_mtoon::MtoonPlugin;
use first_person::SetupFirstPerson;
use loader::{Vrm, VrmLoader};
use serde_vrm::vrm0::FirstPersonFlag;

use crate::spring_bones::SpringBonePlugin;

#[cfg(feature = "animations")]
pub mod animations;
pub mod auto_scene;
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
            .add_event::<SetupFirstPerson>()
            .init_asset::<Vrm>()
            .init_asset_loader::<VrmLoader>()
            .register_type::<BoneName>()
            .register_type::<FirstPersonFlag>()
            .add_systems(
                Update,
                (auto_scene::set_vrm_scene, first_person::handle_setup_events).chain(),
            );
    }
}

#[derive(Component, Default)]
pub struct VrmInstance(pub Handle<Vrm>);

#[derive(Component, Default)]
pub struct VrmScene(pub Handle<Scene>);

#[derive(Bundle, Default)]
pub struct VrmBundle {
    pub auto_scene: AutoScene,
    pub scene: VrmScene,
    pub vrm: VrmInstance,
}
