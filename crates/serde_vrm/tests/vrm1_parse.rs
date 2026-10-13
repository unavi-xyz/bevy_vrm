//! Parses official VRM 1.0 samples and pins schema defaults and migrations.

use serde::de::DeserializeOwned;
use serde_json::{
    Value,
    from_value,
};
use serde_vrm::{
    vrm0::BoneName,
    vrm1::{
        vrmc_materials_mtoon::VrmcMaterialsMtoon,
        vrmc_node_constraint::{
            Constraint,
            VrmcNodeConstraint,
        },
        vrmc_spring_bone::{
            Shape,
            SpringJoint,
            VrmcSpringBone,
        },
        vrmc_vrm::{
            VrmcVrm,
            humanoid::HumanBoneName,
        },
    },
};

const SEED_SAN: &str = include_str!("fixtures/seed_san.json");
const CONSTRAINT_TWIST: &str = include_str!("fixtures/constraint_twist.json");

struct Sample {
    vrm:         VrmcVrm,
    spring_bone: VrmcSpringBone,
    mtoon:       Vec<VrmcMaterialsMtoon>,
    constraints: Vec<VrmcNodeConstraint>,
}

fn parse<T: DeserializeOwned>(value: &Value, what: &str) -> T {
    from_value(value.clone()).unwrap_or_else(|e| panic!("{what} failed to deserialize: {e}"))
}

fn load(json: &str) -> Sample {
    let root: Value = serde_json::from_str(json).expect("fixture is valid json");

    let vrm = parse(&root["extensions"][VrmcVrm::EXTENSION_NAME], "VRMC_vrm");
    let spring_bone = parse(
        &root["extensions"][VrmcSpringBone::EXTENSION_NAME],
        "VRMC_springBone",
    );

    let mut mtoon = Vec::new();
    for (i, material) in root["materials"]
        .as_array()
        .expect("materials")
        .iter()
        .enumerate()
    {
        if let Some(ext) = material.get(VrmcMaterialsMtoon::EXTENSION_NAME) {
            mtoon.push(parse(ext, &format!("materials[{i}] VRMC_materials_mtoon")));
        }
    }

    let mut constraints = Vec::new();
    for (i, node) in root["nodes"].as_array().expect("nodes").iter().enumerate() {
        if let Some(ext) = node.get(VrmcNodeConstraint::EXTENSION_NAME) {
            constraints.push(parse(ext, &format!("nodes[{i}] VRMC_node_constraint")));
        }
    }

    Sample {
        vrm,
        spring_bone,
        mtoon,
        constraints,
    }
}

#[test]
fn official_samples_deserialize() {
    let seed = load(SEED_SAN);
    assert!(
        seed.vrm
            .humanoid
            .human_bones
            .contains_key(&HumanBoneName::Head)
    );
    let first_person = seed.vrm.first_person.expect("seed_san has firstPerson");
    assert_ne!(first_person.mesh_annotations.len(), 0);
    assert!(seed.spring_bone.springs.iter().any(|s| s.joints.len() >= 2));
    assert!(
        seed.spring_bone
            .colliders
            .iter()
            .any(|c| matches!(c.shape, Shape::Capsule(_)))
    );
    assert_ne!(seed.mtoon.len(), 0);

    let twist = load(CONSTRAINT_TWIST);
    assert!(
        twist
            .constraints
            .iter()
            .any(|c| matches!(c.constraint, Constraint::Aim(_)))
    );
    assert!(
        twist
            .constraints
            .iter()
            .any(|c| matches!(c.constraint, Constraint::Roll(_)))
    );
}

#[test]
fn vrm0_thumb_bones_shift_to_vrm1_names() {
    let cases = [
        (
            BoneName::LeftThumbProximal,
            HumanBoneName::LeftThumbMetacarpal,
        ),
        (
            BoneName::LeftThumbIntermediate,
            HumanBoneName::LeftThumbProximal,
        ),
        (BoneName::LeftThumbDistal, HumanBoneName::LeftThumbDistal),
        (
            BoneName::RightThumbProximal,
            HumanBoneName::RightThumbMetacarpal,
        ),
        (
            BoneName::RightThumbIntermediate,
            HumanBoneName::RightThumbProximal,
        ),
        (BoneName::RightThumbDistal, HumanBoneName::RightThumbDistal),
    ];
    for (old, new) in cases {
        assert_eq!(HumanBoneName::from(old), new, "{old:?}");
    }
}

#[test]
fn spring_joint_defaults_match_spec() {
    let joint: SpringJoint = serde_json::from_str(r#"{"node":0}"#).expect("joint");
    assert_eq!(joint.node, 0);
    assert!((joint.hit_radius - 0.0).abs() < f32::EPSILON);
    assert!((joint.stiffness - 1.0).abs() < f32::EPSILON);
    assert!((joint.gravity_power - 0.0).abs() < f32::EPSILON);
    assert_eq!(joint.gravity_dir, [0.0, -1.0, 0.0]);
    assert!((joint.drag_force - 0.5).abs() < f32::EPSILON);
}

#[test]
fn shade_color_defaults_to_black_per_the_readme() {
    let mtoon: VrmcMaterialsMtoon =
        serde_json::from_str(r#"{"specVersion":"1.0"}"#).expect("mtoon");
    assert_eq!(mtoon.shade_color_factor, [0.0; 3]);
    assert_eq!(mtoon.matcap_factor, [1.0; 3]);
}

#[test]
fn shape_tolerates_extras_beside_the_variant_key() {
    let shape: Shape =
        serde_json::from_str(r#"{"sphere":{"radius":1.0},"extras":{"a":1}}"#).expect("shape");
    assert!(matches!(shape, Shape::Sphere(s) if (s.radius - 1.0).abs() < f32::EPSILON));

    assert!(serde_json::from_str::<Shape>(r#"{"extras":{}}"#).is_err());
}
