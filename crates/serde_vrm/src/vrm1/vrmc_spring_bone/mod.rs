//! The `VRMC_springBone` extension.

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VrmcSpringBone {
    pub spec_version:    String,
    #[serde(default)]
    pub colliders:       Vec<Collider>,
    #[serde(default)]
    pub collider_groups: Vec<ColliderGroup>,
    #[serde(default)]
    pub springs:         Vec<Spring>,
}

impl VrmcSpringBone {
    pub const EXTENSION_NAME: &str = "VRMC_springBone";
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct Collider {
    pub node:  u32,
    pub shape: Shape,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", try_from = "ShapeRepr")]
pub enum Shape {
    Sphere(Sphere),
    Capsule(Capsule),
}

/// Lets `extras` and `extensions` sit beside the shape key.
#[derive(Deserialize)]
struct ShapeRepr {
    sphere:  Option<Sphere>,
    capsule: Option<Capsule>,
}

impl TryFrom<ShapeRepr> for Shape {
    type Error = &'static str;

    fn try_from(repr: ShapeRepr) -> Result<Self, Self::Error> {
        match (repr.sphere, repr.capsule) {
            (Some(sphere), None) => Ok(Self::Sphere(sphere)),
            (None, Some(capsule)) => Ok(Self::Capsule(capsule)),
            _ => Err("shape must have exactly one of `sphere` or `capsule`"),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Sphere {
    #[serde(default)]
    pub offset: [f32; 3],
    #[serde(default)]
    pub radius: f32,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Capsule {
    #[serde(default)]
    pub offset: [f32; 3],
    #[serde(default)]
    pub radius: f32,
    #[serde(default)]
    pub tail:   [f32; 3],
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ColliderGroup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name:      Option<String>,
    pub colliders: Vec<u32>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Spring {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name:            Option<String>,
    pub joints:          Vec<SpringJoint>,
    #[serde(default)]
    pub collider_groups: Vec<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub center:          Option<u32>,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpringJoint {
    pub node:          u32,
    #[serde(default)]
    pub hit_radius:    f32,
    #[serde(default = "default_stiffness")]
    pub stiffness:     f32,
    #[serde(default)]
    pub gravity_power: f32,
    #[serde(default = "default_gravity_dir")]
    pub gravity_dir:   [f32; 3],
    #[serde(default = "default_drag_force")]
    pub drag_force:    f32,
}

const fn default_stiffness() -> f32 {
    1.0
}

const fn default_gravity_dir() -> [f32; 3] {
    [0.0, -1.0, 0.0]
}

const fn default_drag_force() -> f32 {
    0.5
}
