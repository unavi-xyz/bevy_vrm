//! First-person mesh visibility.

#[cfg(feature = "bevy")]
use bevy::ecs::reflect::ReflectComponent;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FirstPerson {
    #[serde(default)]
    pub mesh_annotations: Vec<MeshAnnotation>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MeshAnnotation {
    pub node: u32,
    #[serde(rename = "type")]
    pub kind: FirstPersonType,
}

#[cfg_attr(
    feature = "bevy",
    derive(bevy::reflect::Reflect, bevy::prelude::Component),
    reflect(Component)
)]
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum FirstPersonType {
    #[default]
    Auto,
    Both,
    ThirdPersonOnly,
    FirstPersonOnly,
}
