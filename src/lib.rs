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
            .add_systems(Update, replace_mtoon_materials);
    }
}

#[derive(Debug)]
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
    /// Whether MToon materials have been replaced
    pub mtoon_replaced: bool,
}

fn replace_mtoon_materials(
    mut commands: Commands,
    mut vrms: ResMut<Assets<Vrm>>,
    gltf_meshes: Res<Assets<GltfMesh>>,
    meshes: Query<(Entity, &Handle<Mesh>)>,
) {
    for (_, vrm) in vrms.iter_mut() {
        if vrm.mtoon_replaced {
            continue;
        }

        vrm.mtoon_markers.iter().for_each(|marker| {
            let gltf_mesh = gltf_meshes.get(&marker.mesh).unwrap();

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

                        commands
                            .entity(entity)
                            .remove::<Handle<StandardMaterial>>()
                            .insert(marker.mtoon.clone());
                    }
                });
        });

        vrm.mtoon_replaced = true;
    }
}
