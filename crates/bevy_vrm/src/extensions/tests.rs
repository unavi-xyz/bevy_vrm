//! Unit tests for avatar construction and application.

use std::{
    collections::{
        HashMap,
        HashSet,
    },
    f32::consts::{
        FRAC_PI_2,
        PI,
    },
};

use bevy::prelude::*;
use serde_vrm::{
    vrm0,
    vrm1::{
        vrmc_spring_bone::VrmcSpringBone,
        vrmc_vrm::{
            VrmcVrm,
            humanoid::HumanBoneName,
        },
    },
};

use super::{
    avatar::{
        Avatar,
        Facing,
        JointDesc,
        SpringDesc,
        Tail,
    },
    vrm0::spring_from_group,
    vrm1::springs_from_vrm1,
};
use crate::spring_bones::{
    SpringBones,
    SpringSettings,
};

fn load_gltf(file: &str) -> gltf::Gltf {
    let path = format!("{}/../../assets/{file}", env!("CARGO_MANIFEST_DIR"));
    let bytes = std::fs::read(path).expect("asset must be readable");
    gltf::Gltf::from_slice_without_validation(&bytes).expect("asset must be valid glTF")
}

fn settings(stiffness: f32) -> SpringSettings {
    SpringSettings {
        stiffness,
        drag_force: 0.5,
        gravity_power: 0.0,
        gravity_dir: Vec3::NEG_Y,
        hit_radius: 0.0,
    }
}

#[test]
fn vrm0_bone_groups_expand_to_descendants_with_tails() {
    let gltf = load_gltf("alicia.vrm");
    let value = gltf
        .extension_value(vrm0::Vrm::EXTENSION_NAME)
        .expect("alicia.vrm must have a VRM extension");
    let vrm = serde_json::from_value::<vrm0::Vrm>(value.clone()).expect("VRM extension must parse");

    let avatar = Avatar::from_vrm0(&gltf, &vrm);

    assert_eq!(avatar.facing, Facing::NegativeZ);
    assert!(
        avatar
            .human_bones
            .iter()
            .any(|(bone, _)| *bone == HumanBoneName::Head),
        "Head must be in the humanoid"
    );
    assert!(!avatar.springs.is_empty(), "alicia.vrm must have springs");

    let nodes = gltf.nodes().collect::<Vec<_>>();
    for spring in &avatar.springs {
        let chain = spring.joints.iter().map(|j| j.node).collect::<Vec<_>>();

        for (position, joint) in spring.joints.iter().enumerate() {
            let node = &nodes[joint.node];
            let first_child = node.children().next().map(|c| c.index());
            match (joint.tail, first_child) {
                (Tail::Node(tail), Some(child)) => {
                    assert_eq!(tail, child, "tail must be the first child");
                }
                (Tail::Extrapolated, None) => {}
                (tail, child) => panic!("tail {tail:?} does not match first child {child:?}"),
            }

            let parent = nodes
                .iter()
                .find(|n| n.children().any(|c| c.index() == joint.node))
                .map(gltf::Node::index);
            if let Some(parent_position) = parent.and_then(|p| chain.iter().position(|n| *n == p)) {
                assert!(
                    parent_position < position,
                    "node {} must come after its parent",
                    joint.node
                );
            }
        }
    }
}

#[test]
fn vrm1_spring_end_joint_is_only_a_tail() {
    let spring_bone = serde_json::from_value::<VrmcSpringBone>(serde_json::json!({
        "specVersion": "1.0",
        "springs": [
            {
                "joints": [
                    { "node": 1, "stiffness": 0.1, "dragForce": 0.2, "hitRadius": 0.3 },
                    { "node": 2, "stiffness": 0.4, "dragForce": 0.5, "hitRadius": 0.6 },
                    { "node": 3, "stiffness": 0.7, "dragForce": 0.8, "hitRadius": 0.9 }
                ]
            },
            { "joints": [{ "node": 4 }] }
        ]
    }))
    .expect("spring bone JSON must parse");

    let springs = springs_from_vrm1(&spring_bone);

    assert_eq!(
        springs.len(),
        1,
        "a one-joint spring has nothing to simulate"
    );
    let joints = &springs[0].joints;
    assert_eq!(joints.len(), 2);
    assert_eq!((joints[0].node, joints[0].tail), (1, Tail::Node(2)));
    assert_eq!((joints[1].node, joints[1].tail), (2, Tail::Node(3)));
    assert_eq!(joints[0].settings.stiffness, 0.1);
    assert_eq!(joints[0].settings.hit_radius, 0.3);
    assert_eq!(joints[1].settings.stiffness, 0.4);
    assert_eq!(joints[1].settings.drag_force, 0.5);
}

#[test]
fn vrm1_avatar_faces_positive_z_and_maps_humanoid() {
    let gltf = load_gltf("vrm1_mtoon_uv_test.vrm");
    let value = gltf
        .extension_value(VrmcVrm::EXTENSION_NAME)
        .expect("sample must have a VRMC_vrm extension");
    let vrm = serde_json::from_value::<VrmcVrm>(value.clone()).expect("VRMC_vrm must parse");

    let avatar = Avatar::from_vrm1(&gltf, &vrm, None);

    assert_eq!(avatar.facing, Facing::PositiveZ);
    assert!(
        avatar
            .human_bones
            .iter()
            .any(|(bone, _)| *bone == HumanBoneName::Head),
        "Head must be in the humanoid"
    );

    let mesh_nodes = gltf
        .nodes()
        .filter(|n| n.mesh().is_some())
        .map(|n| n.index())
        .collect::<HashSet<_>>();
    let covered = avatar
        .first_person
        .iter()
        .map(|(node, _)| *node)
        .collect::<HashSet<_>>();
    assert!(!mesh_nodes.is_empty());
    assert_eq!(
        covered, mesh_nodes,
        "every mesh node needs a first-person type"
    );
}

#[test]
fn apply_rotates_vrm1_root_and_spawns_extrapolated_tails() {
    let mut world = World::new();
    let root = world.spawn(Transform::default()).id();
    let bone_rotation = Quat::from_rotation_z(FRAC_PI_2);
    let bone = world
        .spawn((
            Transform::from_xyz(0.0, 0.1, 0.0).with_rotation(bone_rotation),
            ChildOf(root),
        ))
        .id();

    let avatar = Avatar {
        facing:       Facing::PositiveZ,
        human_bones:  Vec::new(),
        first_person: Vec::new(),
        springs:      vec![SpringDesc {
            joints: vec![JointDesc {
                node:     0,
                tail:     Tail::Extrapolated,
                settings: settings(1.0),
            }],
        }],
    };

    avatar.apply(&mut world, root, &HashMap::from([(0, bone)]));

    let rotation = world
        .get::<Transform>(root)
        .expect("root has a transform")
        .rotation;
    assert!(
        rotation.angle_between(Quat::from_rotation_y(PI)) < 1.0e-5,
        "root must turn 180 degrees about Y"
    );

    let spring_bones = world
        .get::<SpringBones>(root)
        .expect("root gets SpringBones");
    assert_eq!(spring_bones.0.len(), 1);
    let joint = &spring_bones.0[0].joints[0];
    assert_eq!(joint.bone, bone);

    let tail = world
        .get::<Transform>(joint.tail)
        .expect("tail has a transform");
    assert!(
        tail.translation
            .distance(bone_rotation.inverse() * Vec3::new(0.0, 0.07, 0.0))
            < 1.0e-5,
        "tail extends along the parent-to-bone direction"
    );
    assert_eq!(
        world.get::<ChildOf>(joint.tail).map(ChildOf::parent),
        Some(bone),
        "tail must be a child of the bone"
    );
}

#[test]
fn apply_leaves_vrm0_root_rotation_unchanged() {
    let mut world = World::new();
    let rotation = Quat::from_rotation_x(0.3);
    let root = world.spawn(Transform::from_rotation(rotation)).id();

    let avatar = Avatar {
        facing:       Facing::NegativeZ,
        human_bones:  Vec::new(),
        first_person: Vec::new(),
        springs:      Vec::new(),
    };
    avatar.apply(&mut world, root, &HashMap::new());

    let after = world
        .get::<Transform>(root)
        .expect("root has a transform")
        .rotation;
    assert_eq!(after, rotation, "a -Z avatar must not be rotated");
}

#[test]
fn vrm0_descendant_listed_before_ancestor_still_follows_it() {
    let gltf = gltf::Gltf::from_slice_without_validation(
        br#"{"asset":{"version":"2.0"},"nodes":[{"children":[1]},{"children":[2]},{}]}"#,
    )
    .expect("glTF JSON must parse");
    let group = vrm0::BoneGroup {
        bones: Some(vec![1, 0]),
        ..default()
    };

    let spring = spring_from_group(&gltf, &group);

    let order = spring.joints.iter().map(|j| j.node).collect::<Vec<_>>();
    assert_eq!(order, [0, 1, 2], "parents must precede children");
}
