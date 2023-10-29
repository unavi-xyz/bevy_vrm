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
    pub human_bones: Vec<Bone>,
    #[nserde(rename = "armStretch")]
    pub arm_stretch: f32,
    #[nserde(rename = "legStretch")]
    pub leg_stretch: f32,
    #[nserde(rename = "upperArmTwist")]
    pub upper_arm_twist: f32,
    #[nserde(rename = "lowerArmTwist")]
    pub lower_arm_twist: f32,
    #[nserde(rename = "upperLegTwist")]
    pub upper_leg_twist: f32,
    #[nserde(rename = "lowerLegTwist")]
    pub lower_leg_twist: f32,
    #[nserde(rename = "feetSpacing")]
    pub feet_spacing: f32,
    #[nserde(rename = "hasTranslationDoF")]
    pub has_translation_dof: bool,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct Bone {
    #[nserde(rename = "bone")]
    pub name: String,
    pub node: u32,
    #[nserde(rename = "useDefaultValues")]
    pub use_default_values: bool,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct FirstPerson {
    #[nserde(rename = "firstPersonBone")]
    pub first_person_bone: u32,
    #[nserde(rename = "firstPersonBoneOffset")]
    pub first_person_bone_offset: Vec3,
    #[nserde(rename = "meshAnnotations")]
    pub mesh_annotations: Vec<MeshAnnotation>,
    #[nserde(rename = "lookAtTypeName")]
    pub look_at_type_name: String,
    #[nserde(rename = "lookAtHorizontalInner")]
    pub look_at_horizontal_inner: LookAtCurve,
    #[nserde(rename = "lookAtHorizontalOuter")]
    pub look_at_horizontal_outer: LookAtCurve,
    #[nserde(rename = "lookAtVerticalDown")]
    pub look_at_vertical_down: LookAtCurve,
    #[nserde(rename = "lookAtVerticalUp")]
    pub look_at_vertical_up: LookAtCurve,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct MeshAnnotation {
    pub mesh: u32,
    #[nserde(rename = "firstPersonFlag")]
    pub first_person_flag: String,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct LookAtCurve {
    pub curve: [u32; 8],
    #[nserde(rename = "xRange")]
    pub x_range: u32,
    #[nserde(rename = "yRange")]
    pub y_range: u32,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct BlendShapeMaster {
    #[nserde(rename = "blendShapeGroups")]
    pub blend_shape_groups: Vec<BlendShapeGroup>,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct BlendShapeGroup {
    pub name: String,
    #[nserde(rename = "presetName")]
    pub preset_name: String,
    pub binds: Vec<Bind>,
    #[nserde(rename = "materialValues")]
    pub material_values: Vec<MaterialBind>,
    pub is_binary: bool,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct MaterialBind {
    #[nserde(rename = "materialName")]
    pub material_name: String,
    #[nserde(rename = "propertyName")]
    pub property_name: String,
    #[nserde(rename = "targetValue")]
    pub target_value: Vec<f32>,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct Bind {
    pub mesh: u32,
    pub index: u32,
    pub weight: u32,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct SecondaryAnimation {
    #[nserde(rename = "boneGroups")]
    pub bone_groups: Vec<BoneGroup>,
    #[nserde(rename = "colliderGroups")]
    pub collider_groups: Vec<ColliderGroup>,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct BoneGroup {
    pub comment: String,
    pub stiffiness: f32,
    #[nserde(rename = "gravityPower")]
    pub gravity_power: f32,
    #[nserde(rename = "gravityDir")]
    pub gravity_dir: Vec3,
    #[nserde(rename = "dragForce")]
    pub drag_force: f32,
    pub center: f32,
    #[nserde(rename = "hitRadius")]
    pub hit_radius: f32,
    pub bones: Vec<u32>,
    #[nserde(rename = "colliderGroups")]
    pub collider_groups: Vec<u32>,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct ColliderGroup {
    pub node: u32,
    pub colliders: Vec<Collider>,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct Collider {
    pub offset: Vec3,
    pub radius: f32,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct MaterialProperty {
    pub name: String,
    #[nserde(rename = "renderQueue")]
    pub render_queue: u32,
    pub shader: String,
    #[nserde(rename = "floatProperties")]
    pub float: FloatProperties,
    #[nserde(rename = "vectorProperties")]
    pub vector: VectorProperties,
    #[nserde(rename = "textureProperties")]
    pub texture: TextureProperties,
    #[nserde(rename = "keywordMap")]
    pub keyword_map: KeywordMap,
    #[nserde(rename = "tagMap")]
    pub tag_map: TagMap,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct FloatProperties {
    #[nserde(rename = "_ShadeShift")]
    pub shade_shift: f32,
    #[nserde(rename = "_ShadeToony")]
    pub shade_toony: f32,
    #[nserde(rename = "_Cutoff")]
    pub cutoff: f32,
    #[nserde(rename = "_IndirectLightIntensity")]
    pub indirect_light_insensity: f32,
    #[nserde(rename = "_OutlineWidth")]
    pub outline_width: f32,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct TextureProperties {
    #[nserde(rename = "_MainTex")]
    pub main_tex: u32,
    #[nserde(rename = "_ShadeTexture")]
    pub shade_texture: u32,
    #[nserde(rename = "_BumpMap")]
    pub bump_map: u32,
    #[nserde(rename = "_SphereAdd")]
    pub sphere_add: u32,
    #[nserde(rename = "_EmissionMap")]
    pub emission_map: u32,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct VectorProperties {
    #[nserde(rename = "_Color")]
    pub color: [f32; 4],
    #[nserde(rename = "_ShadeColor")]
    pub shade_color: [f32; 4],
    #[nserde(rename = "_OutlineColor")]
    pub outline_color: [f32; 4],
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct TagMap {
    #[nserde(rename = "RenderType")]
    pub render_type: RenderType,
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
