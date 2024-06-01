//! [Bevy](https://bevyengine.org/) plugin for loading [VRM](https://vrm.dev/en/) avatars.
//! Aims to support both the VRM 0.0 and VRM 1.0 standards.

use bevy::app::PluginGroupBuilder;
use bevy::ecs::entity::MapEntities;
use bevy::ecs::reflect::ReflectMapEntities;
use bevy::prelude::*;
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

use crate::spring_bones::SpringBonePlugin;
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
        app.add_plugins((GltfAssetPlugin, MtoonPlugin))
            .init_asset::<Vrm>()
            .init_asset_loader::<VrmLoader>()
            .register_type::<BoneName>()
            .add_systems(Update, auto_scene::set_vrm_scene);
    }
}

#[derive(Bundle, Default)]
pub struct VrmBundle {
    pub auto_scene: AutoScene,
    pub scene_bundle: SceneBundle,
    pub vrm: Handle<Vrm>,
}

/// Automatically sets the scene to the loaded VRM's default scene.
#[derive(Component, Default)]
pub struct AutoScene;

#[derive(Component, Default, Reflect)]
#[reflect(Component, MapEntities)]
pub struct SpringBones(pub Vec<SpringBone>);

#[derive(Reflect)]
pub struct SpringBone {
    pub bones: Vec<Entity>,
    pub center: f32,
    pub drag_force: f32,
    pub gravity_dir: Vec3,
    pub gravity_power: f32,
    pub hit_radius: f32,
    pub stiffness: f32,
}

impl MapEntities for SpringBone {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        for bone in &mut self.bones {
            *bone = entity_mapper.map_entity(*bone);
        }
    }
}

impl MapEntities for SpringBones {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        for bones in &mut self.0 {
            bones.map_entities(entity_mapper);
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct SpringBoneLogicState {
    pub prev_tail: Vec3,
    pub current_tail: Vec3,
    pub bone_axis: Vec3,
    pub bone_length: f32,
    pub initial_local_matrix: Mat4,
    pub initial_local_rotation: Quat,
}
