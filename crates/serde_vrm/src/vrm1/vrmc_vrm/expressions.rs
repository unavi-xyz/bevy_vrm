//! Facial expressions and their morph target and material binds.

use std::collections::HashMap;

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Expressions {
    #[serde(default)]
    pub preset: Presets,
    #[serde(default)]
    pub custom: HashMap<String, Expression>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Presets {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub happy:       Option<Expression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub angry:       Option<Expression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sad:         Option<Expression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relaxed:     Option<Expression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surprised:   Option<Expression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aa:          Option<Expression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ih:          Option<Expression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ou:          Option<Expression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ee:          Option<Expression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oh:          Option<Expression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blink:       Option<Expression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blink_left:  Option<Expression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blink_right: Option<Expression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub look_up:     Option<Expression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub look_down:   Option<Expression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub look_left:   Option<Expression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub look_right:  Option<Expression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neutral:     Option<Expression>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Expression {
    #[serde(default)]
    pub morph_target_binds:      Vec<MorphTargetBind>,
    #[serde(default)]
    pub material_color_binds:    Vec<MaterialColorBind>,
    #[serde(default)]
    pub texture_transform_binds: Vec<TextureTransformBind>,
    #[serde(default)]
    pub is_binary:               bool,
    #[serde(default)]
    pub override_blink:          ExpressionOverride,
    #[serde(default)]
    pub override_look_at:        ExpressionOverride,
    #[serde(default)]
    pub override_mouth:          ExpressionOverride,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ExpressionOverride {
    #[default]
    None,
    Block,
    Blend,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct MorphTargetBind {
    pub node:   u32,
    pub index:  u32,
    pub weight: f32,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterialColorBind {
    pub material:     u32,
    #[serde(rename = "type")]
    pub kind:         MaterialColorType,
    pub target_value: [f32; 4],
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum MaterialColorType {
    Color,
    EmissionColor,
    ShadeColor,
    MatcapColor,
    RimColor,
    OutlineColor,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct TextureTransformBind {
    pub material: u32,
    #[serde(default = "default_scale")]
    pub scale:    [f32; 2],
    #[serde(default)]
    pub offset:   [f32; 2],
}

const fn default_scale() -> [f32; 2] {
    [1.0, 1.0]
}
