//! VRM 0.0 types.

use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Vrm {
    pub exporter_version: Option<String>,
    pub spec_version: Option<String>,
    pub meta: Option<Meta>,
    pub humanoid: Option<Humanoid>,
    pub first_person: Option<FirstPerson>,
    pub blend_shape_master: Option<BlendShapeMaster>,
    pub secondary_animation: Option<SecondaryAnimation>,
    pub material_properties: Option<Vec<MaterialProperty>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub title: Option<String>,
    pub version: Option<String>,
    pub author: Option<String>,
    pub contact_information: Option<String>,
    pub reference: Option<String>,
    pub texture: Option<u32>,
    pub allowed_user_name: Option<AllowedUserName>,
    pub violent_usage_name: Option<Allow>,
    pub sexual_usage_name: Option<Allow>,
    pub commercial_usage_name: Option<Allow>,
    pub other_permission_url: Option<String>,
    pub license_name: Option<String>,
    pub other_license_url: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum AllowedUserName {
    Everyone,
    ExplicitlyLicensedPerson,
    OnlyAuthor,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Allow {
    Allow,
    Disallow,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Humanoid {
    pub human_bones: Option<Vec<Bone>>,
    pub arm_stretch: Option<f32>,
    pub leg_stretch: Option<f32>,
    pub upper_arm_twist: Option<f32>,
    pub lower_arm_twist: Option<f32>,
    pub upper_leg_twist: Option<f32>,
    pub lower_leg_twist: Option<f32>,
    pub feet_spacing: Option<f32>,
    pub has_translation_dof: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Bone {
    pub bone: Option<BoneName>,
    pub node: Option<u32>,
    pub use_default_values: Option<bool>,
}

#[cfg(feature = "bevy")]
use bevy::ecs::reflect::ReflectComponent;

#[cfg_attr(
    feature = "bevy",
    derive(bevy::reflect::Reflect, bevy::prelude::Component),
    reflect(Component)
)]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum BoneName {
    #[serde(rename = "hips")]
    Hips,
    #[serde(rename = "leftUpperLeg")]
    LeftUpperLeg,
    #[serde(rename = "rightUpperLeg")]
    RightUpperLeg,
    #[serde(rename = "leftLowerLeg")]
    LeftLowerLeg,
    #[serde(rename = "rightLowerLeg")]
    RightLowerLeg,
    #[serde(rename = "leftFoot")]
    LeftFoot,
    #[serde(rename = "rightFoot")]
    RightFoot,
    #[serde(rename = "spine")]
    Spine,
    #[serde(rename = "chest")]
    Chest,
    #[serde(rename = "neck")]
    Neck,
    #[serde(rename = "head")]
    Head,
    #[serde(rename = "leftShoulder")]
    LeftShoulder,
    #[serde(rename = "rightShoulder")]
    RightShoulder,
    #[serde(rename = "leftUpperArm")]
    LeftUpperArm,
    #[serde(rename = "rightUpperArm")]
    RightUpperArm,
    #[serde(rename = "leftLowerArm")]
    LeftLowerArm,
    #[serde(rename = "rightLowerArm")]
    RightLowerArm,
    #[serde(rename = "leftHand")]
    LeftHand,
    #[serde(rename = "rightHand")]
    RightHand,
    #[serde(rename = "leftToes")]
    LeftToes,
    #[serde(rename = "rightToes")]
    RightToes,
    #[serde(rename = "leftEye")]
    LeftEye,
    #[serde(rename = "rightEye")]
    RightEye,
    #[serde(rename = "jaw")]
    Jaw,
    #[serde(rename = "leftThumbProximal")]
    LeftThumbProximal,
    #[serde(rename = "leftThumbIntermediate")]
    LeftThumbIntermediate,
    #[serde(rename = "leftThumbDistal")]
    LeftThumbDistal,
    #[serde(rename = "leftIndexProximal")]
    LeftIndexProximal,
    #[serde(rename = "leftIndexIntermediate")]
    LeftIndexIntermediate,
    #[serde(rename = "leftIndexDistal")]
    LeftIndexDistal,
    #[serde(rename = "leftMiddleProximal")]
    LeftMiddleProximal,
    #[serde(rename = "leftMiddleIntermediate")]
    LeftMiddleIntermediate,
    #[serde(rename = "leftMiddleDistal")]
    LeftMiddleDistal,
    #[serde(rename = "leftRingProximal")]
    LeftRingProximal,
    #[serde(rename = "leftRingIntermediate")]
    LeftRingIntermediate,
    #[serde(rename = "leftRingDistal")]
    LeftRingDistal,
    #[serde(rename = "leftLittleProximal")]
    LeftLittleProximal,
    #[serde(rename = "leftLittleIntermediate")]
    LeftLittleIntermediate,
    #[serde(rename = "leftLittleDistal")]
    LeftLittleDistal,
    #[serde(rename = "rightThumbProximal")]
    RightThumbProximal,
    #[serde(rename = "rightThumbIntermediate")]
    RightThumbIntermediate,
    #[serde(rename = "rightThumbDistal")]
    RightThumbDistal,
    #[serde(rename = "rightIndexProximal")]
    RightIndexProximal,
    #[serde(rename = "rightIndexIntermediate")]
    RightIndexIntermediate,
    #[serde(rename = "rightIndexDistal")]
    RightIndexDistal,
    #[serde(rename = "rightMiddleProximal")]
    RightMiddleProximal,
    #[serde(rename = "rightMiddleIntermediate")]
    RightMiddleIntermediate,
    #[serde(rename = "rightMiddleDistal")]
    RightMiddleDistal,
    #[serde(rename = "rightRingProximal")]
    RightRingProximal,
    #[serde(rename = "rightRingIntermediate")]
    RightRingIntermediate,
    #[serde(rename = "rightRingDistal")]
    RightRingDistal,
    #[serde(rename = "rightLittleProximal")]
    RightLittleProximal,
    #[serde(rename = "rightLittleIntermediate")]
    RightLittleIntermediate,
    #[serde(rename = "rightLittleDistal")]
    RightLittleDistal,
    #[serde(rename = "upperChest")]
    UpperChest,
}

impl Display for BoneName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&serde_json::to_string(self).unwrap())
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FirstPerson {
    pub first_person_bone: Option<u32>,
    pub first_person_bone_offset: Option<Vec3>,
    pub mesh_annotations: Option<Vec<MeshAnnotation>>,
    pub look_at_type_name: Option<String>,
    pub look_at_horizontal_inner: Option<LookAtCurve>,
    pub look_at_horizontal_outer: Option<LookAtCurve>,
    pub look_at_vertical_down: Option<LookAtCurve>,
    pub look_at_vertical_up: Option<LookAtCurve>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MeshAnnotation {
    pub mesh: Option<u32>,
    pub first_person_flag: FirstPersonFlag,
}

#[derive(Copy, Clone, Debug, Default, Hash, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum FirstPersonFlag {
    #[default]
    #[serde(alias = "Auto")]
    Auto,
    Both,
    FirstPersonOnly,
    ThirdPersonOnly,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LookAtCurve {
    pub curve: Option<[f32; 8]>,
    pub x_range: Option<f32>,
    pub y_range: Option<f32>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlendShapeMaster {
    pub blend_shape_groups: Option<Vec<BlendShapeGroup>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlendShapeGroup {
    pub name: Option<String>,
    pub preset_name: Option<PresetName>,
    pub binds: Option<Vec<Bind>>,
    pub material_values: Option<Vec<MaterialBind>>,
    pub is_binary: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum PresetName {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "neutral")]
    Neutral,
    #[serde(rename = "a")]
    A,
    #[serde(rename = "i")]
    I,
    #[serde(rename = "u")]
    U,
    #[serde(rename = "e")]
    E,
    #[serde(rename = "o")]
    O,
    #[serde(rename = "blink")]
    Blink,
    #[serde(rename = "joy")]
    Joy,
    #[serde(rename = "angry")]
    Angry,
    #[serde(rename = "sorrow")]
    Sorrow,
    #[serde(rename = "fun")]
    Fun,
    #[serde(rename = "lookup")]
    LookUp,
    #[serde(rename = "lookdown")]
    LookDown,
    #[serde(rename = "lookleft")]
    LookLeft,
    #[serde(rename = "lookright")]
    LookRight,
    #[serde(rename = "blink_l")]
    BlinkLeft,
    #[serde(rename = "blink_r")]
    BlinkRight,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterialBind {
    pub material_name: Option<String>,
    pub property_name: Option<String>,
    pub target_value: Option<Vec<f32>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Bind {
    pub mesh: Option<u32>,
    pub index: Option<u32>,
    pub weight: Option<f32>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SecondaryAnimation {
    pub bone_groups: Option<Vec<BoneGroup>>,
    pub collider_groups: Option<Vec<ColliderGroup>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BoneGroup {
    pub comment: Option<String>,
    pub stiffiness: Option<f32>,
    pub gravity_power: Option<f32>,
    pub gravity_dir: Option<Vec3>,
    pub drag_force: Option<f32>,
    pub center: Option<f32>,
    pub hit_radius: Option<f32>,
    pub bones: Option<Vec<u32>>,
    pub collider_groups: Option<Vec<u32>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ColliderGroup {
    pub node: Option<u32>,
    pub colliders: Option<Vec<Collider>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Collider {
    pub offset: Option<Vec3>,
    pub radius: Option<f32>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MaterialProperty {
    pub name: Option<String>,
    #[serde(rename = "renderQueue")]
    pub render_queue: Option<i32>,
    pub shader: Option<Shader>,
    #[serde(rename = "floatProperties")]
    pub float: Option<FloatProperties>,
    #[serde(rename = "vectorProperties")]
    pub vector: Option<VectorProperties>,
    #[serde(rename = "textureProperties")]
    pub texture: Option<TextureProperties>,
    #[serde(rename = "keywordMap")]
    pub keyword_map: Option<KeywordMap>,
    #[serde(rename = "tagMap")]
    pub tag_map: Option<TagMap>,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Shader {
    #[serde(rename = "VRM_USE_GLTFSHADER")]
    Gltf,
    #[serde(rename = "VRM/MToon")]
    MToon,
    #[serde(rename = "VRM/UnlitCutout")]
    UnlitCutout,
    #[serde(rename = "VRM/UnlitTexture")]
    UnlitTexture,
    #[serde(rename = "VRM/UnlitTransparent")]
    UnlitTransparent,
    #[serde(rename = "VRM/UnlitTransparentZWrite")]
    UnlitTransparentZWrite,
    Other(String),
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FloatProperties {
    #[serde(rename = "_ShadeShift")]
    pub shade_shift: Option<f32>,
    #[serde(rename = "_ShadeToony")]
    pub shade_toony: Option<f32>,
    #[serde(rename = "_Cutoff")]
    pub cutoff: Option<f32>,
    #[serde(rename = "_IndirectLightIntensity")]
    pub gi_intensity_factor: Option<f32>,
    #[serde(rename = "_BumpScale")]
    pub normal_scale: Option<f32>,
    #[serde(rename = "_CullMode")]
    pub double_sided: Option<f32>,
    #[serde(rename = "_ReceiveShadowRate")]
    pub shade_receive_multiply_factor: Option<f32>,
    #[serde(rename = "_RimLightingMix")]
    pub rim_lighting_mix_factor: Option<f32>,
    #[serde(rename = "_RimFresnelPower")]
    pub rim_fresnel_power_factor: Option<f32>,
    #[serde(rename = "_RimLift")]
    pub rim_lift_factor: Option<f32>,
    #[serde(rename = "_OutlineWidth")]
    pub outline_factor: Option<f32>,
    #[serde(rename = "_OutlineWidthMode")]
    pub outline_width_mode: Option<f32>,
    #[serde(rename = "_OutlineScaledMaxDistance")]
    pub outline_scaled_max_distance_factor: Option<f32>,
    #[serde(rename = "_OutlineLightingMix")]
    pub outline_lighting_mix_factor: Option<f32>,
    #[serde(rename = "_UvAnimScrollX")]
    pub uv_animation_scroll_x_speed_factor: Option<f32>,
    #[serde(rename = "_UvAnimScrollY")]
    pub uv_animation_scroll_y_speed_factor: Option<f32>,
    #[serde(rename = "_UvAnimRotation")]
    pub uv_animation_rotation_speed_factor: Option<f32>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TextureProperties {
    #[serde(rename = "_MainTex")]
    pub base_color: Option<u32>,
    #[serde(rename = "_ShadeTexture")]
    pub shade: Option<u32>,
    #[serde(rename = "_BumpMap")]
    pub normal: Option<u32>,
    #[serde(rename = "_SphereAdd")]
    pub additive: Option<u32>,
    #[serde(rename = "_EmissionMap")]
    pub emissive: Option<u32>,
    #[serde(rename = "_RimTexture")]
    pub rim_multiply: Option<u32>,
    #[serde(rename = "_OutlineWidthTexture")]
    pub outline_width_multiply_texture: Option<u32>,
    #[serde(rename = "_UvAnimMaskTexture")]
    pub uv_animation_mask_texture: Option<u32>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VectorProperties {
    #[serde(rename = "_Color")]
    pub color: Option<[f32; 4]>,
    #[serde(rename = "_EmissionColor")]
    pub emissive_factor: Option<[f32; 4]>,
    #[serde(rename = "_OutlineColor")]
    pub outline_color: Option<[f32; 4]>,
    #[serde(rename = "_ShadeColor")]
    pub shade_color: Option<[f32; 4]>,
    #[serde(rename = "_RimColor")]
    pub rim_factor: Option<[f32; 4]>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TagMap {
    #[serde(rename = "RenderType")]
    pub render_type: Option<RenderType>,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum RenderType {
    Opaque,
    Transparent,
    TransparentCutout,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct KeywordMap {
    #[serde(rename = "_ALPHABLEND_ON")]
    pub alpha_blend: Option<bool>,
    #[serde(rename = "_ALPHATEST_ON")]
    pub alpha_test: Option<bool>,
    #[serde(rename = "_NORMALMAP")]
    pub normal_map: Option<bool>,
    #[serde(rename = "MTOON_OUTLINE_COLOR_FIXED")]
    pub outline_color_fixed: Option<bool>,
    #[serde(rename = "MTOON_OUTLINE_COLOR_MIXED")]
    pub outline_color_mixed: Option<bool>,
    #[serde(rename = "MTOON_OUTLINE_WIDTH_WORLD")]
    pub outline_width_world: Option<bool>,
}
