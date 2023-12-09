use bevy::{gltf::GltfMesh, prelude::*, utils::HashMap};
use bevy_shader_mtoon::{MtoonMaterial, MtoonPlugin};
use loader::VrmLoader;

mod extensions;
pub mod loader;

pub mod mtoon {
    pub use bevy_shader_mtoon::*;
}

pub struct VrmPlugin;

impl Plugin for VrmPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MtoonPlugin)
            .register_asset_loader(VrmLoader)
            .init_asset::<Vrm>()
            .add_systems(
                Update,
                (set_vrm_scene, spawn_mtoon_markers, replace_mtoon_materials),
            );
    }
}

#[derive(Component, Debug, Clone)]
pub struct MtoonMarker {
    mesh: Handle<GltfMesh>,
    primitive: usize,
    mtoon: Handle<MtoonMaterial>,
}

#[derive(Asset, TypePath, Debug)]
pub struct Vrm {
    pub gltf: bevy::gltf::Gltf,
    /// Map of material index -> MToon material
    pub mtoon_materials: HashMap<usize, Handle<MtoonMaterial>>,
    /// Meshes that use MToon
    pub mtoon_markers: Vec<MtoonMarker>,
}

#[derive(Bundle, Default)]
pub struct VrmBundle {
    pub vrm: Handle<Vrm>,
    pub scene_bundle: SceneBundle,
}

fn set_vrm_scene(
    mut commands: Commands,
    vrm: Res<Assets<Vrm>>,
    scenes: Query<(Entity, &mut Handle<Scene>, &Handle<Vrm>)>,
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

        if scene_handle.id() == vrm_scene.id() {
            continue;
        }

        commands.entity(entity).insert(vrm_scene.clone());
    }
}

fn spawn_mtoon_markers(
    mut commands: Commands,
    vrms: Res<Assets<Vrm>>,
    mut events: EventReader<AssetEvent<Vrm>>,
) {
    for event in events.read() {
        let id = match event {
            AssetEvent::Added { id } => id,
            AssetEvent::Modified { id } => id,
            _ => continue,
        };

        let vrm = match vrms.get(*id) {
            Some(vrm) => vrm,
            None => continue,
        };

        vrm.mtoon_markers.iter().for_each(|marker| {
            commands.spawn(marker.clone());
        });
    }
}

fn replace_mtoon_materials(
    mut commands: Commands,
    markers: Query<(Entity, &MtoonMarker)>,
    gltf_meshes: Res<Assets<GltfMesh>>,
    meshes: Query<(Entity, &Handle<Mesh>)>,
) {
    for (entity, marker) in markers.iter() {
        let gltf_mesh = gltf_meshes.get(&marker.mesh).unwrap();

        let mut replaced = false;

        gltf_mesh
            .primitives
            .iter()
            .enumerate()
            .for_each(|(primitive_index, primitive)| {
                if primitive_index != marker.primitive {
                    return;
                }

                for (entity, mesh_handle) in meshes.iter() {
                    if mesh_handle.id() != primitive.mesh.id() {
                        continue;
                    }

                    replaced = true;

                    commands
                        .entity(entity)
                        .remove::<Handle<StandardMaterial>>()
                        .insert(marker.mtoon.clone());
                }
            });

        if replaced {
            commands.entity(entity).remove::<MtoonMarker>();
        }
    }
}
