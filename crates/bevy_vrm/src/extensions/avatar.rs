//! Version-neutral avatar description and its application to a spawned scene.

use std::{
    collections::{
        HashMap,
        HashSet,
    },
    f32::consts::PI,
};

use bevy::prelude::*;
use serde_vrm::vrm1::vrmc_vrm::{
    first_person::FirstPersonType,
    humanoid::HumanBoneName,
};

use crate::spring_bones::{
    SpringBones,
    SpringChain,
    SpringJoint,
    SpringSettings,
};

/// Length of the virtual tail added to a spring leaf, in meters.
const EXTRAPOLATED_TAIL_LENGTH: f32 = 0.07;

/// Node indices and normalized data read from a VRM file.
#[derive(Clone, Debug, PartialEq)]
pub struct Avatar {
    pub facing:       Facing,
    pub human_bones:  Vec<(HumanBoneName, usize)>,
    /// One entry for every glTF node that has a mesh.
    pub first_person: Vec<(usize, FirstPersonType)>,
    pub springs:      Vec<SpringDesc>,
}

/// Direction the avatar faces in glTF space. 0.0 exporters face -Z; 1.0
/// follows glTF (+Z).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Facing {
    NegativeZ,
    PositiveZ,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SpringDesc {
    pub joints: Vec<JointDesc>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct JointDesc {
    pub node:     usize,
    pub tail:     Tail,
    pub settings: SpringSettings,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tail {
    Node(usize),
    /// A leaf with no tail node; a 7 cm virtual tail is created on apply.
    Extrapolated,
}

impl Avatar {
    pub fn apply(&self, world: &mut World, root: Entity, nodes: &HashMap<usize, Entity>) {
        if self.facing == Facing::PositiveZ
            && let Some(mut transform) = world.get_mut::<Transform>(root)
        {
            transform.rotation = Quat::from_rotation_y(PI) * transform.rotation;
        }

        for (bone, node) in &self.human_bones {
            if let Some(&entity) = nodes.get(node) {
                setup_bone(world, entity, *bone);
            }
        }

        for (node, ty) in &self.first_person {
            if let Some(&entity) = nodes.get(node) {
                insert_first_person(world, entity, *ty);
            }
        }

        let chains = self.build_chains(world, nodes);
        if !chains.is_empty()
            && let Ok(mut root) = world.get_entity_mut(root)
        {
            root.insert(SpringBones(chains));
        }
    }

    fn build_chains(&self, world: &mut World, nodes: &HashMap<usize, Entity>) -> Vec<SpringChain> {
        let mut chains = Vec::new();

        for spring in &self.springs {
            let mut joints = Vec::new();

            for joint in &spring.joints {
                let Some(&bone) = nodes.get(&joint.node) else {
                    continue;
                };
                let tail = match joint.tail {
                    Tail::Node(node) => nodes.get(&node).copied(),
                    Tail::Extrapolated => spawn_extrapolated_tail(world, bone),
                };
                let Some(tail) = tail else {
                    continue;
                };

                joints.push(SpringJoint {
                    bone,
                    tail,
                    settings: joint.settings,
                });
            }

            if !joints.is_empty() {
                chains.push(SpringChain { joints });
            }
        }

        chains
    }
}

/// Resolves the first-person type of every mesh node. An unannotated mesh at
/// or below the head is third-person only.
pub fn resolve_first_person(
    gltf: &gltf::Gltf,
    head: Option<usize>,
    annotated: &HashMap<usize, FirstPersonType>,
) -> Vec<(usize, FirstPersonType)> {
    let nodes = gltf.nodes().collect::<Vec<_>>();

    let mut head_subtree = HashSet::new();
    let mut stack = head.into_iter().collect::<Vec<_>>();
    while let Some(index) = stack.pop() {
        if !head_subtree.insert(index) {
            continue;
        }
        if let Some(node) = nodes.get(index) {
            stack.extend(node.children().map(|child| child.index()));
        }
    }

    let mut resolved = Vec::new();
    for node in &nodes {
        if node.mesh().is_none() {
            continue;
        }

        let index = node.index();
        let mut ty = annotated.get(&index).copied().unwrap_or_default();
        if ty == FirstPersonType::Auto && head_subtree.contains(&index) {
            ty = FirstPersonType::ThirdPersonOnly;
        }
        resolved.push((index, ty));
    }

    resolved
}

fn setup_bone(world: &mut World, entity: Entity, bone: HumanBoneName) {
    let Ok(mut bone_entity) = world.get_entity_mut(entity) else {
        return;
    };
    bone_entity.insert(bone);

    #[cfg(feature = "animations")]
    {
        let mut root = entity;
        while let Some(child_of) = world.get::<ChildOf>(root) {
            root = child_of.parent();
        }

        if let Ok(mut root_entity) = world.get_entity_mut(root) {
            root_entity.insert(bevy::animation::AnimationPlayer::default());
        }

        if let Some(target) = crate::animations::vrm::VRM_ANIMATION_TARGETS.get(&bone)
            && let Ok(mut bone_entity) = world.get_entity_mut(entity)
        {
            bone_entity.insert((*target, bevy::animation::AnimatedBy(root)));
        }
    }
}

fn insert_first_person(world: &mut World, node: Entity, ty: FirstPersonType) {
    let Some(children) = world.get::<Children>(node) else {
        return;
    };
    let children = children.iter().collect::<Vec<_>>();

    for child in children {
        if world.get::<Mesh3d>(child).is_none() {
            continue;
        }
        if let Ok(mut child) = world.get_entity_mut(child) {
            child.insert(ty);
        }
    }
}

/// Extends the bone along its parent-to-bone direction, as `UniVRM` does.
fn spawn_extrapolated_tail(world: &mut World, bone: Entity) -> Option<Entity> {
    let local = *world.get::<Transform>(bone)?;

    let direction = local.rotation.inverse() * local.translation.normalize_or_zero();
    let offset = if direction == Vec3::ZERO {
        Vec3::new(0.0, -EXTRAPOLATED_TAIL_LENGTH, 0.0)
    } else {
        direction * EXTRAPOLATED_TAIL_LENGTH
    };

    Some(
        world
            .spawn((Transform::from_translation(offset), ChildOf(bone)))
            .id(),
    )
}
