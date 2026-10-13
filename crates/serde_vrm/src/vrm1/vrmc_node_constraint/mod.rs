//! The `VRMC_node_constraint` extension.

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VrmcNodeConstraint {
    pub spec_version: String,
    pub constraint:   Constraint,
}

impl VrmcNodeConstraint {
    pub const EXTENSION_NAME: &str = "VRMC_node_constraint";
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", try_from = "ConstraintRepr")]
pub enum Constraint {
    Roll(RollConstraint),
    Aim(AimConstraint),
    Rotation(RotationConstraint),
}

/// Lets `extras` and `extensions` sit beside the constraint key.
#[derive(Deserialize)]
struct ConstraintRepr {
    roll:     Option<RollConstraint>,
    aim:      Option<AimConstraint>,
    rotation: Option<RotationConstraint>,
}

impl TryFrom<ConstraintRepr> for Constraint {
    type Error = &'static str;

    fn try_from(repr: ConstraintRepr) -> Result<Self, Self::Error> {
        match (repr.roll, repr.aim, repr.rotation) {
            (Some(roll), None, None) => Ok(Self::Roll(roll)),
            (None, Some(aim), None) => Ok(Self::Aim(aim)),
            (None, None, Some(rotation)) => Ok(Self::Rotation(rotation)),
            _ => Err("constraint must have exactly one of `roll`, `aim` or `rotation`"),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RollConstraint {
    pub source:    u32,
    pub roll_axis: RollAxis,
    #[serde(default = "default_weight")]
    pub weight:    f32,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AimConstraint {
    pub source:   u32,
    pub aim_axis: AimAxis,
    #[serde(default = "default_weight")]
    pub weight:   f32,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct RotationConstraint {
    pub source: u32,
    #[serde(default = "default_weight")]
    pub weight: f32,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum RollAxis {
    X,
    Y,
    Z,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum AimAxis {
    PositiveX,
    NegativeX,
    PositiveY,
    NegativeY,
    PositiveZ,
    NegativeZ,
}

const fn default_weight() -> f32 {
    1.0
}
