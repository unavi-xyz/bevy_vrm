use std::collections::{
    HashMap,
    HashSet,
};

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
use serde_vrm::vrm0::{
    BoneName,
    FirstPersonFlag,
    MaterialProperty,
    Shader,
    Vrm,
};

use crate::spring_bones::{
    SpringBone,
    SpringBones,
};

pub mod mtoon;

#[derive(Default, Clone)]
pub struct VrmHandler {
    vrm:                 Option<Vrm>,
    node_entities:       HashMap<usize, Entity>,
    third_person_meshes: HashSet<usize>,
}

impl VrmHandler {
    fn material_props(&self, index: Option<usize>) -> Option<&MaterialProperty> {
        let index = index?;
        self.vrm.as_ref()?.material_properties.as_ref()?.get(index)
    }

    fn first_person_flag(&self, mesh_index: usize) -> FirstPersonFlag {
        let mut flag = self
            .vrm
            .as_ref()
            .and_then(|v| v.first_person.as_ref())
            .and_then(|fp| fp.mesh_annotations.as_ref())
            .and_then(|anns| anns.iter().find(|a| a.mesh == Some(mesh_index as u32)))
            .map(|a| a.first_person_flag)
            .unwrap_or_default();

        if flag == FirstPersonFlag::Auto && self.third_person_meshes.contains(&mesh_index) {
            flag = FirstPersonFlag::ThirdPersonOnly;
        }

        flag
    }
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
        let Some(value) = gltf.extensions().and_then(|e| e.get("VRM")) else {
            return;
        };

        match serde_json::from_value::<Vrm>(value.clone()) {
            Ok(vrm) => {
                self.third_person_meshes = third_person_meshes(gltf, &vrm);
                self.vrm = Some(vrm);
            }
            Err(err) => warn!("Failed to parse VRM extension: {err}"),
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
        let Some(props) = self.material_props(gltf_material.index()) else {
            return;
        };

        if props.shader != Some(Shader::MToon) {
            return;
        }

        let material = mtoon::build_mtoon_material(load_context, props);
        load_context.add_labeled_asset(mtoon_label(material_label), material);
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
        mesh: &Mesh,
        material: &Material,
        entity: &mut EntityWorldMut,
        material_label: &str,
    ) {
        if self.vrm.is_none() {
            return;
        }

        if let Some(props) = self.material_props(material.index())
            && props.shader == Some(Shader::MToon)
        {
            let handle =
                load_context.get_label_handle::<MtoonMaterial>(mtoon_label(material_label));
            entity.remove::<MeshMaterial3d<StandardMaterial>>();
            entity.insert((MeshMaterial3d(handle), OutlineSync));
        }

        let flag = self.first_person_flag(mesh.index());
        entity.insert(flag);
    }

    fn on_scene_completed(
        &mut self,
        _load_context: &mut LoadContext<'_>,
        _scene: &Scene,
        world_root_id: Entity,
        scene_world: &mut World,
    ) {
        let Some(vrm) = self.vrm.clone() else {
            return;
        };

        let mut spring_bones = vec![];

        if let Some(groups) = vrm
            .secondary_animation
            .as_ref()
            .and_then(|sa| sa.bone_groups.as_ref())
        {
            for group in groups {
                let mut bones = vec![];
                let mut bone_names = vec![];

                for node in group.bones.iter().flatten() {
                    let Some(&entity) = self.node_entities.get(&(*node as usize)) else {
                        continue;
                    };
                    let Some(name) = scene_world.get::<Name>(entity) else {
                        continue;
                    };
                    bones.push(entity);
                    bone_names.push(name.to_string());
                }

                let gravity_dir = group
                    .gravity_dir
                    .as_ref()
                    .map(|d| Vec3::new(d.x, d.y, d.z))
                    .unwrap_or_default();

                spring_bones.push(SpringBone {
                    bones,
                    bone_names,
                    center: group.center.unwrap_or_default(),
                    drag_force: group.drag_force.unwrap_or_default(),
                    gravity_dir,
                    gravity_power: group.gravity_power.unwrap_or_default(),
                    hit_radius: group.hit_radius.unwrap_or_default(),
                    stiffness: group.stiffiness.unwrap_or_default(),
                });
            }
        }

        if !spring_bones.is_empty() {
            scene_world
                .entity_mut(world_root_id)
                .insert(SpringBones(spring_bones));
        }

        if let Some(bones) = vrm.humanoid.as_ref().and_then(|h| h.human_bones.as_ref()) {
            for bone in bones {
                let (Some(node), Some(bone_name)) = (bone.node, bone.bone) else {
                    continue;
                };
                let Some(&entity) = self.node_entities.get(&(node as usize)) else {
                    continue;
                };
                setup_bone(scene_world, entity, bone_name);
            }
        }
    }
}

fn setup_bone(world: &mut World, entity: Entity, bone_name: BoneName) {
    world.entity_mut(entity).insert(bone_name);

    #[cfg(feature = "animations")]
    {
        let mut root = entity;
        while let Some(child_of) = world.get::<ChildOf>(root) {
            root = child_of.parent();
        }

        world
            .entity_mut(root)
            .insert(bevy::animation::AnimationPlayer::default());

        if let Some(target) = crate::animations::vrm::VRM_ANIMATION_TARGETS.get(&bone_name) {
            world
                .entity_mut(entity)
                .insert((*target, bevy::animation::AnimatedBy(root)));
        }
    }
}

fn third_person_meshes(gltf: &gltf::Gltf, vrm: &Vrm) -> HashSet<usize> {
    let mut meshes = HashSet::new();

    let Some(head_node) = vrm
        .humanoid
        .as_ref()
        .and_then(|h| h.human_bones.as_ref())
        .and_then(|bones| bones.iter().find(|b| b.bone == Some(BoneName::Head)))
        .and_then(|b| b.node)
    else {
        return meshes;
    };

    let nodes = gltf.nodes().collect::<Vec<_>>();

    let mut descendants = HashSet::new();
    let mut stack = vec![head_node as usize];
    while let Some(idx) = stack.pop() {
        if !descendants.insert(idx) {
            continue;
        }
        if let Some(node) = nodes.get(idx) {
            for child in node.children() {
                stack.push(child.index());
            }
        }
    }

    for node in &nodes {
        if descendants.contains(&node.index())
            && let Some(mesh) = node.mesh()
        {
            meshes.insert(mesh.index());
        }
    }

    meshes
}

fn mtoon_label(material_label: &str) -> String {
    format!("{material_label}/mtoon")
}
