//! glTF extension handler reading VRM 0.0 and VRM 1.0 data.

use std::collections::{
    HashMap,
    HashSet,
};

use avatar::Avatar;
use bevy::{
    asset::LoadContext,
    gltf::{
        GltfLoaderSettings,
        extensions::{
            ErasedGltfExtensionHandler,
            GltfExtensionHandler,
        },
    },
    prelude::*,
};
use bevy_shader_mtoon::{
    MtoonMaterial,
    OutlineSync,
};
use gltf::{
    Material,
    Mesh,
    Node,
    Primitive,
    Scene,
};
use serde_vrm::{
    vrm0::{
        MaterialProperty,
        Shader,
        Vrm as Vrm0,
    },
    vrm1::{
        vrmc_materials_mtoon::VrmcMaterialsMtoon,
        vrmc_spring_bone::VrmcSpringBone,
        vrmc_vrm::VrmcVrm,
    },
};

mod avatar;
mod mtoon;
#[cfg(test)] mod tests;
mod vrm0;
mod vrm1;

#[derive(Default, Clone)]
pub struct VrmHandler {
    avatar:          Option<Avatar>,
    vrm0_materials:  Vec<MaterialProperty>,
    mtoon_materials: HashSet<usize>,
    node_entities:   HashMap<usize, Entity>,
}

impl GltfExtensionHandler for VrmHandler {
    fn dyn_clone(&self) -> Box<dyn ErasedGltfExtensionHandler> {
        Box::new(self.clone())
    }

    fn on_root(
        &mut self,
        _load_context: &mut LoadContext<'_>,
        gltf: &gltf::Gltf,
        _settings: &GltfLoaderSettings,
    ) {
        if let Some(value) = gltf.extension_value(VrmcVrm::EXTENSION_NAME) {
            match serde_json::from_value::<VrmcVrm>(value.clone()) {
                Ok(vrm) => {
                    let springs = parse_springs(gltf);
                    self.avatar = Some(Avatar::from_vrm1(gltf, &vrm, springs.as_ref()));
                }
                Err(err) => warn!(
                    "Failed to parse {} extension: {err}",
                    VrmcVrm::EXTENSION_NAME
                ),
            }
        } else if let Some(value) = gltf.extension_value(Vrm0::EXTENSION_NAME) {
            match serde_json::from_value::<Vrm0>(value.clone()) {
                Ok(vrm) => {
                    self.avatar = Some(Avatar::from_vrm0(gltf, &vrm));
                    self.vrm0_materials = vrm.material_properties.unwrap_or_default();
                }
                Err(err) => warn!("Failed to parse {} extension: {err}", Vrm0::EXTENSION_NAME),
            }
        }
    }

    fn on_material(
        &mut self,
        load_context: &mut LoadContext<'_>,
        gltf_material: &Material,
        _material: Handle<bevy::gltf::GltfMaterial>,
        _material_asset: &bevy::gltf::GltfMaterial,
        material_label: &str,
    ) {
        if self.avatar.is_none() {
            return;
        }
        let Some(index) = gltf_material.index() else {
            return;
        };

        let mtoon = if let Some(value) =
            gltf_material.extension_value(VrmcMaterialsMtoon::EXTENSION_NAME)
        {
            match serde_json::from_value::<VrmcMaterialsMtoon>(value.clone()) {
                Ok(extension) => Some(mtoon::vrm1::from_vrm1(
                    load_context,
                    gltf_material,
                    &extension,
                )),
                Err(err) => {
                    warn!(
                        "Failed to parse {} extension: {err}",
                        VrmcMaterialsMtoon::EXTENSION_NAME
                    );
                    None
                }
            }
        } else {
            self.vrm0_materials
                .get(index)
                .filter(|props| props.shader == Some(Shader::MToon))
                .map(|props| mtoon::vrm0::from_vrm0(load_context, props))
        };

        if let Some(mtoon) = mtoon {
            load_context.add_labeled_asset(mtoon_label(material_label), mtoon);
            self.mtoon_materials.insert(index);
        }
    }

    fn on_gltf_node(
        &mut self,
        _load_context: &mut LoadContext<'_>,
        gltf_node: &Node,
        entity: &mut EntityWorldMut,
    ) {
        self.node_entities.insert(gltf_node.index(), entity.id());
    }

    fn on_spawn_mesh_and_material(
        &mut self,
        load_context: &mut LoadContext<'_>,
        _primitive: &Primitive,
        _mesh: &Mesh,
        material: &Material,
        entity: &mut EntityWorldMut,
        material_label: &str,
    ) {
        if material
            .index()
            .is_some_and(|index| self.mtoon_materials.contains(&index))
        {
            let handle =
                load_context.get_label_handle::<MtoonMaterial>(mtoon_label(material_label));
            entity.remove::<MeshMaterial3d<StandardMaterial>>();
            entity.insert((MeshMaterial3d(handle), OutlineSync));
        }
    }

    fn on_scene_completed(
        &mut self,
        _load_context: &mut LoadContext<'_>,
        _scene: &Scene,
        world_root_id: Entity,
        scene_world: &mut World,
    ) {
        if let Some(avatar) = &self.avatar {
            avatar.apply(scene_world, world_root_id, &self.node_entities);
        }
        self.node_entities.clear();
    }
}

fn mtoon_label(material_label: &str) -> String {
    format!("{material_label}/mtoon")
}

fn parse_springs(gltf: &gltf::Gltf) -> Option<VrmcSpringBone> {
    let value = gltf.extension_value(VrmcSpringBone::EXTENSION_NAME)?;

    match serde_json::from_value::<VrmcSpringBone>(value.clone()) {
        Ok(springs) => Some(springs),
        Err(err) => {
            warn!(
                "Failed to parse {} extension: {err}",
                VrmcSpringBone::EXTENSION_NAME
            );
            None
        }
    }
}
