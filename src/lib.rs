use bevy::{
    gltf::GltfMesh,
    prelude::*,
    reflect::{TypePath, TypeUuid},
};
use bevy_shader_mtoon::MtoonPlugin;

mod extensions;
pub mod loader;

pub mod mtoon {
    pub use bevy_shader_mtoon::*;
}

pub struct VrmPlugin;

impl Plugin for VrmPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MtoonPlugin)
            .add_asset::<MtoonReplaceMat>()
            .init_asset_loader::<loader::VrmLoader>()
            .add_systems(Update, replace_mtoon_materials);
    }
}

#[derive(TypeUuid, TypePath, Debug, Clone)]
#[uuid = "e16c60c0-fa00-4ead-b76f-f97823b8404e"]
pub struct MtoonReplaceMat {
    mesh: Handle<GltfMesh>,
    primitive: usize,
    mtoon: Handle<mtoon::MtoonMaterial>,
}

fn replace_mtoon_materials(
    mut commands: Commands,
    mut replacements: ResMut<Assets<MtoonReplaceMat>>,
    gltf_meshes: Res<Assets<GltfMesh>>,
    meshes: Query<(Entity, &Handle<Mesh>), Without<Handle<mtoon::MtoonMaterial>>>,
) {
    let mut to_remove = Vec::new();

    for (replacement_handle, replacement) in replacements.iter() {
        let target = gltf_meshes.get(&replacement.mesh).unwrap();

        target
            .primitives
            .iter()
            .enumerate()
            .for_each(|(i, primitive)| {
                if i != replacement.primitive {
                    return;
                }

                for (ent, ent_mesh) in meshes.iter() {
                    if *ent_mesh != primitive.mesh {
                        continue;
                    }

                    commands
                        .entity(ent)
                        .remove::<Handle<StandardMaterial>>()
                        .insert(replacement.mtoon.clone());

                    to_remove.push(replacement_handle);
                    break;
                }
            });
    }

    for handle in to_remove {
        replacements.remove(handle);
    }
}
