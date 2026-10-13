//! The `VRMC_vrm` extension.

pub mod expressions;
pub mod first_person;
pub mod humanoid;
pub mod look_at;
pub mod meta;

use serde::{
    Deserialize,
    Serialize,
};

use self::{
    expressions::Expressions,
    first_person::FirstPerson,
    humanoid::Humanoid,
    look_at::LookAt,
    meta::Meta,
};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VrmcVrm {
    pub spec_version: String,
    pub meta:         Meta,
    pub humanoid:     Humanoid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_person: Option<FirstPerson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub look_at:      Option<LookAt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expressions:  Option<Expressions>,
}

impl VrmcVrm {
    pub const EXTENSION_NAME: &str = "VRMC_vrm";
}
