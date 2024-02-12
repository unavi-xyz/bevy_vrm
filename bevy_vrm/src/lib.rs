use bevy::gltf::GltfNode;
use bevy::scene::SceneInstance;
use bevy::{gltf::GltfMesh, prelude::*, utils::HashMap};
use bevy_shader_mtoon::{MtoonMaterial, MtoonPlugin};
use loader::{RootExtensions, VrmLoader};

pub mod extensions;
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
                (
                    set_vrm_scene,
                    spawn_mtoon_markers,
                    replace_mtoon_materials,
                    vrm_bone_hook,
                ),
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
    /// GLTF vrm extension info.
    pub extensions: RootExtensions,
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

#[derive(Component)]
pub struct VrmLoaded;

pub struct SpringBone {
    pub stiffiness: f32,
    pub gravity_power: f32,
    pub gravity_dir: Vec3,
    pub drag_force: f32,
    pub center: f32,
    pub hit_radius: f32,
    pub bones: Vec<Entity>,
}

#[derive(Component)]
pub struct SpringBones(pub Vec<SpringBone>);

#[derive(Component)]
pub struct HumanoidBones {
    pub neck: Entity,
    pub head: Entity,
    pub left_eye: Option<Entity>,
    pub right_eye: Option<Entity>,
    pub jaw: Option<Entity>,
    pub hips: Entity,
    pub spine: Entity,
    pub chest: Entity,
    pub upper_chest: Option<Entity>,
    pub left_shoulder: Option<Entity>,
    pub right_shoulder: Option<Entity>,
    pub left_upper_arm: Entity,
    pub right_upper_arm: Entity,
    pub left_lower_arm: Entity,
    pub right_lower_arm: Entity,
    pub left_hand: Entity,
    pub right_hand: Entity,
    pub left_upper_leg: Entity,
    pub right_upper_leg: Entity,
    pub left_lower_leg: Entity,
    pub right_lower_leg: Entity,
    pub left_foot: Entity,
    pub right_foot: Entity,
    pub left_toe: Option<Entity>,
    pub right_toe: Option<Entity>,
    pub left_thumb_proximal: Option<Entity>,
    pub right_thumb_proximal: Option<Entity>,
    pub left_thumb_intermediate: Option<Entity>,
    pub right_thumb_intermediate: Option<Entity>,
    pub left_thumb_distal: Option<Entity>,
    pub right_thumb_distal: Option<Entity>,
    pub left_index_proximal: Option<Entity>,
    pub right_index_proximal: Option<Entity>,
    pub left_index_intermediate: Option<Entity>,
    pub right_index_intermediate: Option<Entity>,
    pub left_index_distal: Option<Entity>,
    pub right_index_distal: Option<Entity>,
    pub left_middle_proximal: Option<Entity>,
    pub right_middle_proximal: Option<Entity>,
    pub left_middle_intermediate: Option<Entity>,
    pub right_middle_intermediate: Option<Entity>,
    pub left_middle_distal: Option<Entity>,
    pub right_middle_distal: Option<Entity>,
    pub left_ring_proximal: Option<Entity>,
    pub right_ring_proximal: Option<Entity>,
    pub left_ring_intermediate: Option<Entity>,
    pub right_ring_intermediate: Option<Entity>,
    pub left_ring_distal: Option<Entity>,
    pub right_ring_distal: Option<Entity>,
    pub left_little_proximal: Option<Entity>,
    pub right_little_proximal: Option<Entity>,
    pub left_little_intermediate: Option<Entity>,
    pub right_little_intermediate: Option<Entity>,
    pub left_little_distal: Option<Entity>,
    pub right_little_distal: Option<Entity>,
    pub others: std::collections::HashMap<String, Entity>,
}

#[derive(Default)]
struct UnfinishedHumanoidBones {
    neck: Option<Entity>,
    head: Option<Entity>,
    left_eye: Option<Entity>,
    right_eye: Option<Entity>,
    jaw: Option<Entity>,
    hips: Option<Entity>,
    spine: Option<Entity>,
    chest: Option<Entity>,
    upper_chest: Option<Entity>,
    left_shoulder: Option<Entity>,
    right_shoulder: Option<Entity>,
    left_upper_arm: Option<Entity>,
    right_upper_arm: Option<Entity>,
    left_lower_arm: Option<Entity>,
    right_lower_arm: Option<Entity>,
    left_hand: Option<Entity>,
    right_hand: Option<Entity>,
    left_upper_leg: Option<Entity>,
    right_upper_leg: Option<Entity>,
    left_lower_leg: Option<Entity>,
    right_lower_leg: Option<Entity>,
    left_foot: Option<Entity>,
    right_foot: Option<Entity>,
    left_toe: Option<Entity>,
    right_toe: Option<Entity>,
    left_thumb_proximal: Option<Entity>,
    right_thumb_proximal: Option<Entity>,
    left_thumb_intermediate: Option<Entity>,
    right_thumb_intermediate: Option<Entity>,
    left_thumb_distal: Option<Entity>,
    right_thumb_distal: Option<Entity>,
    left_index_proximal: Option<Entity>,
    right_index_proximal: Option<Entity>,
    left_index_intermediate: Option<Entity>,
    right_index_intermediate: Option<Entity>,
    left_index_distal: Option<Entity>,
    right_index_distal: Option<Entity>,
    left_middle_proximal: Option<Entity>,
    right_middle_proximal: Option<Entity>,
    left_middle_intermediate: Option<Entity>,
    right_middle_intermediate: Option<Entity>,
    left_middle_distal: Option<Entity>,
    right_middle_distal: Option<Entity>,
    left_ring_proximal: Option<Entity>,
    right_ring_proximal: Option<Entity>,
    left_ring_intermediate: Option<Entity>,
    right_ring_intermediate: Option<Entity>,
    left_ring_distal: Option<Entity>,
    right_ring_distal: Option<Entity>,
    left_little_proximal: Option<Entity>,
    right_little_proximal: Option<Entity>,
    left_little_intermediate: Option<Entity>,
    right_little_intermediate: Option<Entity>,
    left_little_distal: Option<Entity>,
    right_little_distal: Option<Entity>,
    others: std::collections::HashMap<String, Entity>,
}

fn vrm_bone_hook(
    unloaded_instances: Query<(Entity, &SceneInstance, &Handle<Vrm>), Without<VrmLoaded>>,
    scene_manager: Res<SceneSpawner>,
    vrm_assets: Res<Assets<Vrm>>,
    names: Query<&Name>,
    mut commands: Commands,
) {
    for (entity, instance, vrm) in unloaded_instances.iter() {
        if scene_manager.instance_is_ready(**instance) && vrm_assets.contains(vrm) {
            commands.entity(entity).insert(VrmLoaded);
        } else {
            continue;
        }
        let mut unfinished_humanoid_bones = UnfinishedHumanoidBones::default();
        let vrm = vrm_assets.get(vrm).unwrap();
        if let Some(vrm0) = vrm.extensions.vrm0.as_ref() {
            if let Some(secondary_animation) = vrm0.secondary_animation.as_ref() {
                if let Some(bone_groups) = secondary_animation.bone_groups.as_ref() {
                    let mut spring_bones = SpringBones(vec![]);
                    for bone_group in bone_groups {
                        let bones = bone_group.bones.as_ref().unwrap();
                        let mut actual_bones = vec![];
                        for bone_node in bones {
                            let bone_handle: &Handle<GltfNode> =
                                vrm.gltf.nodes.get(*bone_node as usize).unwrap();

                            let mut bone_name = None;
                            for (name, temp_handle) in vrm.gltf.named_nodes.iter() {
                                if temp_handle == bone_handle {
                                    bone_name.replace(name);
                                    break;
                                }
                            }
                            let bone_name = bone_name.unwrap();

                            let mut actual_entity = None;
                            for child in scene_manager.iter_instance_entities(**instance) {
                                if let Ok(name) = names.get(child) {
                                    if name.as_str() == bone_name {
                                        actual_entity.replace(child);
                                        break;
                                    }
                                }
                            }
                            let actual_entity = actual_entity.unwrap();
                            actual_bones.push(actual_entity);
                        }
                        spring_bones.0.push(SpringBone {
                            stiffiness: bone_group.stiffiness.unwrap(),
                            gravity_power: bone_group.gravity_power.unwrap(),
                            gravity_dir: Vec3::new(
                                bone_group.gravity_dir.as_ref().unwrap().x,
                                bone_group.gravity_dir.as_ref().unwrap().y,
                                bone_group.gravity_dir.as_ref().unwrap().z,
                            ),
                            drag_force: bone_group.drag_force.unwrap(),
                            center: bone_group.center.unwrap(),
                            hit_radius: bone_group.hit_radius.unwrap(),
                            bones: actual_bones,
                        })
                    }
                    commands.entity(entity).insert(spring_bones);
                }
            }
            if let Some(bones) = vrm0.humanoid.human_bones.as_ref() {
                for bone in bones {
                    let bone_name = bone.name.as_ref().unwrap();
                    let bone_node = bone.node.unwrap();
                    let bone_handle: &Handle<GltfNode> =
                        vrm.gltf.nodes.get(bone_node as usize).unwrap();

                    let mut actual_bone_name = None;
                    for (name, temp_handle) in vrm.gltf.named_nodes.iter() {
                        if temp_handle == bone_handle {
                            actual_bone_name.replace(name);
                            break;
                        }
                    }
                    let actual_bone_name = actual_bone_name.unwrap();
                    let mut actual_entity = None;
                    for child in scene_manager.iter_instance_entities(**instance) {
                        if let Ok(name) = names.get(child) {
                            if name.as_str() == actual_bone_name {
                                actual_entity.replace(child);
                                break;
                            }
                        }
                    }
                    let actual_entity = actual_entity.unwrap();

                    match bone_name.as_str() {
                        "neck" => unfinished_humanoid_bones.neck.replace(actual_entity),
                        "head" => unfinished_humanoid_bones.head.replace(actual_entity),
                        "leftEye" => unfinished_humanoid_bones.left_eye.replace(actual_entity),
                        "rightEye" => unfinished_humanoid_bones.right_eye.replace(actual_entity),
                        "jaw" => unfinished_humanoid_bones.jaw.replace(actual_entity),
                        "hips" => unfinished_humanoid_bones.hips.replace(actual_entity),
                        "spine" => unfinished_humanoid_bones.spine.replace(actual_entity),
                        "chest" => unfinished_humanoid_bones.chest.replace(actual_entity),
                        "upperChest" => {
                            unfinished_humanoid_bones.upper_chest.replace(actual_entity)
                        }
                        "leftShoulder" => unfinished_humanoid_bones
                            .left_shoulder
                            .replace(actual_entity),
                        "rightShoulder" => unfinished_humanoid_bones
                            .right_shoulder
                            .replace(actual_entity),
                        "leftUpperArm" => unfinished_humanoid_bones
                            .left_upper_arm
                            .replace(actual_entity),
                        "rightUpperArm" => unfinished_humanoid_bones
                            .right_upper_arm
                            .replace(actual_entity),
                        "leftLowerArm" => unfinished_humanoid_bones
                            .left_lower_arm
                            .replace(actual_entity),
                        "rightLowerArm" => unfinished_humanoid_bones
                            .right_lower_arm
                            .replace(actual_entity),
                        "leftHand" => unfinished_humanoid_bones.left_hand.replace(actual_entity),
                        "rightHand" => unfinished_humanoid_bones.right_hand.replace(actual_entity),
                        "leftUpperLeg" => unfinished_humanoid_bones
                            .left_upper_leg
                            .replace(actual_entity),
                        "rightUpperLeg" => unfinished_humanoid_bones
                            .right_upper_leg
                            .replace(actual_entity),
                        "leftLowerLeg" => unfinished_humanoid_bones
                            .left_lower_leg
                            .replace(actual_entity),
                        "rightLowerLeg" => unfinished_humanoid_bones
                            .right_lower_leg
                            .replace(actual_entity),
                        "leftFoot" => unfinished_humanoid_bones.left_foot.replace(actual_entity),
                        "rightFoot" => unfinished_humanoid_bones.right_foot.replace(actual_entity),
                        "leftToe" => unfinished_humanoid_bones.left_toe.replace(actual_entity),
                        "rightToe" => unfinished_humanoid_bones.right_toe.replace(actual_entity),
                        "leftThumbProximal" => unfinished_humanoid_bones
                            .left_thumb_proximal
                            .replace(actual_entity),
                        "rightThumbProximal" => unfinished_humanoid_bones
                            .right_thumb_proximal
                            .replace(actual_entity),
                        "leftThumbIntermediate" => unfinished_humanoid_bones
                            .left_thumb_intermediate
                            .replace(actual_entity),
                        "rightThumbIntermediate" => unfinished_humanoid_bones
                            .right_thumb_intermediate
                            .replace(actual_entity),
                        "leftThumbDistal" => unfinished_humanoid_bones
                            .left_thumb_distal
                            .replace(actual_entity),
                        "rightThumbDistal" => unfinished_humanoid_bones
                            .right_thumb_distal
                            .replace(actual_entity),
                        "leftIndexProximal" => unfinished_humanoid_bones
                            .left_index_proximal
                            .replace(actual_entity),
                        "rightIndexProximal" => unfinished_humanoid_bones
                            .right_index_proximal
                            .replace(actual_entity),
                        "leftIndexIntermediate" => unfinished_humanoid_bones
                            .left_index_intermediate
                            .replace(actual_entity),
                        "rightIndexIntermediate" => unfinished_humanoid_bones
                            .right_index_intermediate
                            .replace(actual_entity),
                        "leftIndexDistal" => unfinished_humanoid_bones
                            .left_index_distal
                            .replace(actual_entity),
                        "rightIndexDistal" => unfinished_humanoid_bones
                            .right_index_distal
                            .replace(actual_entity),
                        "leftMiddleProximal" => unfinished_humanoid_bones
                            .left_middle_proximal
                            .replace(actual_entity),
                        "rightMiddleProximal" => unfinished_humanoid_bones
                            .right_middle_proximal
                            .replace(actual_entity),
                        "leftMiddleIntermediate" => unfinished_humanoid_bones
                            .left_middle_intermediate
                            .replace(actual_entity),
                        "rightMiddleIntermediate" => unfinished_humanoid_bones
                            .right_middle_intermediate
                            .replace(actual_entity),
                        "leftMiddleDistal" => unfinished_humanoid_bones
                            .left_middle_distal
                            .replace(actual_entity),
                        "rightMiddleDistal" => unfinished_humanoid_bones
                            .right_middle_distal
                            .replace(actual_entity),
                        "leftRingProximal" => unfinished_humanoid_bones
                            .left_ring_proximal
                            .replace(actual_entity),
                        "rightRingProximal" => unfinished_humanoid_bones
                            .right_ring_proximal
                            .replace(actual_entity),
                        "leftRingIntermediate" => unfinished_humanoid_bones
                            .left_ring_intermediate
                            .replace(actual_entity),
                        "rightRingIntermediate" => unfinished_humanoid_bones
                            .right_ring_intermediate
                            .replace(actual_entity),
                        "leftRingDistal" => unfinished_humanoid_bones
                            .left_ring_distal
                            .replace(actual_entity),
                        "rightRingDistal" => unfinished_humanoid_bones
                            .right_ring_distal
                            .replace(actual_entity),
                        "leftLittleProximal" => unfinished_humanoid_bones
                            .left_little_proximal
                            .replace(actual_entity),
                        "rightLittleProximal" => unfinished_humanoid_bones
                            .right_little_proximal
                            .replace(actual_entity),
                        "leftLittleIntermediate" => unfinished_humanoid_bones
                            .left_little_intermediate
                            .replace(actual_entity),
                        "rightLittleIntermediate" => unfinished_humanoid_bones
                            .right_little_intermediate
                            .replace(actual_entity),
                        "leftLittleDistal" => unfinished_humanoid_bones
                            .left_little_distal
                            .replace(actual_entity),
                        "rightLittleDistal" => unfinished_humanoid_bones
                            .right_little_distal
                            .replace(actual_entity),
                        others => unfinished_humanoid_bones
                            .others
                            .insert(others.to_string(), actual_entity),
                    };
                }
                commands.entity(entity).insert(HumanoidBones {
                    neck: unfinished_humanoid_bones.neck.unwrap(),
                    head: unfinished_humanoid_bones.head.unwrap(),
                    left_eye: unfinished_humanoid_bones.left_eye,
                    right_eye: unfinished_humanoid_bones.right_eye,
                    jaw: unfinished_humanoid_bones.jaw,
                    hips: unfinished_humanoid_bones.hips.unwrap(),
                    spine: unfinished_humanoid_bones.spine.unwrap(),
                    chest: unfinished_humanoid_bones.chest.unwrap(),
                    upper_chest: unfinished_humanoid_bones.upper_chest,
                    left_shoulder: unfinished_humanoid_bones.left_shoulder,
                    right_shoulder: unfinished_humanoid_bones.right_shoulder,
                    left_upper_arm: unfinished_humanoid_bones.left_upper_arm.unwrap(),
                    right_upper_arm: unfinished_humanoid_bones.right_upper_arm.unwrap(),
                    left_lower_arm: unfinished_humanoid_bones.left_lower_arm.unwrap(),
                    right_lower_arm: unfinished_humanoid_bones.right_lower_arm.unwrap(),
                    left_hand: unfinished_humanoid_bones.left_hand.unwrap(),
                    right_hand: unfinished_humanoid_bones.right_hand.unwrap(),
                    left_upper_leg: unfinished_humanoid_bones.left_upper_leg.unwrap(),
                    right_upper_leg: unfinished_humanoid_bones.right_upper_leg.unwrap(),
                    left_lower_leg: unfinished_humanoid_bones.left_lower_leg.unwrap(),
                    right_lower_leg: unfinished_humanoid_bones.right_lower_leg.unwrap(),
                    left_foot: unfinished_humanoid_bones.left_foot.unwrap(),
                    right_foot: unfinished_humanoid_bones.right_foot.unwrap(),
                    left_toe: unfinished_humanoid_bones.left_toe,
                    right_toe: unfinished_humanoid_bones.right_toe,
                    left_thumb_proximal: unfinished_humanoid_bones.left_thumb_proximal,
                    right_thumb_proximal: unfinished_humanoid_bones.right_thumb_proximal,
                    left_thumb_intermediate: unfinished_humanoid_bones.left_thumb_intermediate,
                    right_thumb_intermediate: unfinished_humanoid_bones.right_thumb_intermediate,
                    left_thumb_distal: unfinished_humanoid_bones.left_thumb_distal,
                    right_thumb_distal: unfinished_humanoid_bones.right_thumb_distal,
                    left_index_proximal: unfinished_humanoid_bones.left_index_proximal,
                    right_index_proximal: unfinished_humanoid_bones.right_index_proximal,
                    left_index_intermediate: unfinished_humanoid_bones.left_index_intermediate,
                    right_index_intermediate: unfinished_humanoid_bones.right_index_intermediate,
                    left_index_distal: unfinished_humanoid_bones.left_index_distal,
                    right_index_distal: unfinished_humanoid_bones.right_index_distal,
                    left_middle_proximal: unfinished_humanoid_bones.left_middle_proximal,
                    right_middle_proximal: unfinished_humanoid_bones.right_middle_proximal,
                    left_middle_intermediate: unfinished_humanoid_bones.left_middle_intermediate,
                    right_middle_intermediate: unfinished_humanoid_bones.right_middle_intermediate,
                    left_middle_distal: unfinished_humanoid_bones.left_middle_distal,
                    right_middle_distal: unfinished_humanoid_bones.right_middle_distal,
                    left_ring_proximal: unfinished_humanoid_bones.left_ring_proximal,
                    right_ring_proximal: unfinished_humanoid_bones.right_ring_proximal,
                    left_ring_intermediate: unfinished_humanoid_bones.left_ring_intermediate,
                    right_ring_intermediate: unfinished_humanoid_bones.right_ring_intermediate,
                    left_ring_distal: unfinished_humanoid_bones.left_ring_distal,
                    right_ring_distal: unfinished_humanoid_bones.right_ring_distal,
                    left_little_proximal: unfinished_humanoid_bones.left_little_proximal,
                    right_little_proximal: unfinished_humanoid_bones.right_little_proximal,
                    left_little_intermediate: unfinished_humanoid_bones.left_little_intermediate,
                    right_little_intermediate: unfinished_humanoid_bones.right_little_intermediate,
                    left_little_distal: unfinished_humanoid_bones.left_little_distal,
                    right_little_distal: unfinished_humanoid_bones.right_little_distal,
                    others: unfinished_humanoid_bones.others,
                });
            }
        }
    }
}
