use nanoserde::{DeJson, SerJson};

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct Vrm {
    #[nserde(rename = "exporterVersion")]
    pub exporter_version: Option<String>,
    #[nserde(rename = "specVersion")]
    pub spec_version: Option<String>,
    pub meta: Option<Meta>,
    pub humanoid: Humanoid,
    #[nserde(rename = "firstPerson")]
    pub first_person: Option<FirstPerson>,
    #[nserde(rename = "blendShapeMaster")]
    pub blend_shape_master: Option<BlendShapeMaster>,
    #[nserde(rename = "secondaryAnimation")]
    pub secondary_animation: Option<SecondaryAnimation>,
    #[nserde(rename = "materialProperties")]
    pub material_properties: Option<Vec<MaterialProperty>>,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct Meta {
    pub title: Option<String>,
    pub version: Option<String>,
    pub author: Option<String>,
    #[nserde(rename = "contactInformation")]
    pub contact_information: Option<String>,
    pub reference: Option<String>,
    pub texture: Option<u32>,
    #[nserde(rename = "allowedUserName")]
    pub allowed_user_name: Option<String>,
    #[nserde(rename = "violentUssageName")]
    pub violent_usage_name: Option<String>,
    #[nserde(rename = "sexualUssageName")]
    pub sexual_usage_name: Option<String>,
    #[nserde(rename = "commercialUssageName")]
    pub commercial_usage_name: Option<String>,
    #[nserde(rename = "otherPermissionUrl")]
    pub other_permission_url: Option<String>,
    #[nserde(rename = "licenseName")]
    pub license_name: Option<String>,
    #[nserde(rename = "otherLicenseUrl")]
    pub other_license_url: Option<String>,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct Humanoid {
    #[nserde(rename = "humanBones")]
    pub human_bones: Option<Vec<Bone>>,
    #[nserde(rename = "armStretch")]
    pub arm_stretch: Option<f32>,
    #[nserde(rename = "legStretch")]
    pub leg_stretch: Option<f32>,
    #[nserde(rename = "upperArmTwist")]
    pub upper_arm_twist: Option<f32>,
    #[nserde(rename = "lowerArmTwist")]
    pub lower_arm_twist: Option<f32>,
    #[nserde(rename = "upperLegTwist")]
    pub upper_leg_twist: Option<f32>,
    #[nserde(rename = "lowerLegTwist")]
    pub lower_leg_twist: Option<f32>,
    #[nserde(rename = "feetSpacing")]
    pub feet_spacing: Option<f32>,
    #[nserde(rename = "hasTranslationDoF")]
    pub has_translation_dof: Option<bool>,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct Bone {
    #[nserde(rename = "bone")]
    pub name: Option<String>,
    pub node: Option<u32>,
    #[nserde(rename = "useDefaultValues")]
    pub use_default_values: Option<bool>,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct FirstPerson {
    #[nserde(rename = "firstPersonBone")]
    pub first_person_bone: Option<u32>,
    #[nserde(rename = "firstPersonBoneOffset")]
    pub first_person_bone_offset: Option<Vec3>,
    #[nserde(rename = "meshAnnotations")]
    pub mesh_annotations: Option<Vec<MeshAnnotation>>,
    #[nserde(rename = "lookAtTypeName")]
    pub look_at_type_name: Option<String>,
    #[nserde(rename = "lookAtHorizontalInner")]
    pub look_at_horizontal_inner: Option<LookAtCurve>,
    #[nserde(rename = "lookAtHorizontalOuter")]
    pub look_at_horizontal_outer: Option<LookAtCurve>,
    #[nserde(rename = "lookAtVerticalDown")]
    pub look_at_vertical_down: Option<LookAtCurve>,
    #[nserde(rename = "lookAtVerticalUp")]
    pub look_at_vertical_up: Option<LookAtCurve>,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct MeshAnnotation {
    pub mesh: Option<u32>,
    #[nserde(rename = "firstPersonFlag")]
    pub first_person_flag: Option<String>,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct LookAtCurve {
    pub curve: Option<[f32; 8]>,
    #[nserde(rename = "xRange")]
    pub x_range: Option<f32>,
    #[nserde(rename = "yRange")]
    pub y_range: Option<f32>,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct BlendShapeMaster {
    #[nserde(rename = "blendShapeGroups")]
    pub blend_shape_groups: Option<Vec<BlendShapeGroup>>,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct BlendShapeGroup {
    pub name: Option<String>,
    #[nserde(rename = "presetName")]
    pub preset_name: Option<String>,
    pub binds: Option<Vec<Bind>>,
    #[nserde(rename = "materialValues")]
    pub material_values: Option<Vec<MaterialBind>>,
    #[nserde(rename = "isBinary")]
    pub is_binary: Option<bool>,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct MaterialBind {
    #[nserde(rename = "materialName")]
    pub material_name: Option<String>,
    #[nserde(rename = "propertyName")]
    pub property_name: Option<String>,
    #[nserde(rename = "targetValue")]
    pub target_value: Option<Vec<f32>>,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct Bind {
    pub mesh: Option<u32>,
    pub index: Option<u32>,
    pub weight: Option<f32>,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct SecondaryAnimation {
    #[nserde(rename = "boneGroups")]
    pub bone_groups: Option<Vec<BoneGroup>>,
    #[nserde(rename = "colliderGroups")]
    pub collider_groups: Option<Vec<ColliderGroup>>,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct BoneGroup {
    pub comment: Option<String>,
    pub stiffiness: Option<f32>,
    #[nserde(rename = "gravityPower")]
    pub gravity_power: Option<f32>,
    #[nserde(rename = "gravityDir")]
    pub gravity_dir: Option<Vec3>,
    #[nserde(rename = "dragForce")]
    pub drag_force: Option<f32>,
    pub center: Option<f32>,
    #[nserde(rename = "hitRadius")]
    pub hit_radius: Option<f32>,
    pub bones: Option<Vec<u32>>,
    #[nserde(rename = "colliderGroups")]
    pub collider_groups: Option<Vec<u32>>,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct ColliderGroup {
    pub node: Option<u32>,
    pub colliders: Option<Vec<Collider>>,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct Collider {
    pub offset: Option<Vec3>,
    pub radius: Option<f32>,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct MaterialProperty {
    pub name: Option<String>,
    #[nserde(rename = "renderQueue")]
    pub render_queue: Option<i32>,
    pub shader: Option<String>,
    #[nserde(rename = "floatProperties")]
    pub float: Option<FloatProperties>,
    #[nserde(rename = "vectorProperties")]
    pub vector: Option<VectorProperties>,
    #[nserde(rename = "textureProperties")]
    pub texture: Option<TextureProperties>,
    #[nserde(rename = "keywordMap")]
    pub keyword_map: Option<KeywordMap>,
    #[nserde(rename = "tagMap")]
    pub tag_map: Option<TagMap>,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct FloatProperties {
    #[nserde(rename = "_ShadeShift")]
    pub shade_shift: Option<f32>,
    #[nserde(rename = "_ShadeToony")]
    pub shade_toony: Option<f32>,
    #[nserde(rename = "_Cutoff")]
    pub cutoff: Option<f32>,
    #[nserde(rename = "_IndirectLightIntensity")]
    pub indirect_light_insensity: Option<f32>,
    #[nserde(rename = "_OutlineWidth")]
    pub outline_width: Option<f32>,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct TextureProperties {
    #[nserde(rename = "_MainTex")]
    pub main_tex: Option<u32>,
    #[nserde(rename = "_ShadeTexture")]
    pub shade_texture: Option<u32>,
    #[nserde(rename = "_BumpMap")]
    pub bump_map: Option<u32>,
    #[nserde(rename = "_SphereAdd")]
    pub sphere_add: Option<u32>,
    #[nserde(rename = "_EmissionMap")]
    pub emission_map: Option<u32>,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct VectorProperties {
    #[nserde(rename = "_Color")]
    pub color: Option<[f32; 4]>,
    #[nserde(rename = "_ShadeColor")]
    pub shade_color: Option<[f32; 4]>,
    #[nserde(rename = "_OutlineColor")]
    pub outline_color: Option<[f32; 4]>,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct TagMap {
    #[nserde(rename = "RenderType")]
    pub render_type: Option<RenderType>,
}

#[derive(Clone, Debug, DeJson, SerJson)]
pub enum RenderType {
    Transparent,
    TransparentCutout,
    Opaque,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct KeywordMap {
    #[nserde(rename = "_ALPHABLEND_ON")]
    pub alpha_blend: Option<bool>,
    #[nserde(rename = "_ALPHATEST_ON")]
    pub alpha_test: Option<bool>,
    #[nserde(rename = "_NORMALMAP")]
    pub normal_map: Option<bool>,
    #[nserde(rename = "MTOON_OUTLINE_COLOR_FIXED")]
    pub outline_color_fixed: Option<bool>,
    #[nserde(rename = "MTOON_OUTLINE_COLOR_MIXED")]
    pub outline_color_mixed: Option<bool>,
    #[nserde(rename = "MTOON_OUTLINE_WIDTH_WORLD")]
    pub outline_width_world: Option<bool>,
}
