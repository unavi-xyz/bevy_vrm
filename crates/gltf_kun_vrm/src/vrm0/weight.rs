use serde::{Deserialize, Serialize};
use serde_vrm::vrm0::{Allow, AllowedUserName, BoneName, LookAtCurve, Vec3};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VrmWeight {
    pub exporter_version: String,
    pub meta: Meta,
    pub humanoid: Humanoid,
    pub first_person: FirstPerson,
    // pub blend_shape_master: Option<BlendShapeMaster>,
    // pub secondary_animation: Option<SecondaryAnimation>,
    // pub material_properties: Option<Vec<MaterialProperty>>,
}

impl From<&Vec<u8>> for VrmWeight {
    fn from(bytes: &Vec<u8>) -> Self {
        if bytes.is_empty() {
            return Self::default();
        }
        serde_json::from_slice(bytes).expect("Failed to deserialize weight")
    }
}

impl From<&VrmWeight> for Vec<u8> {
    fn from(value: &VrmWeight) -> Self {
        serde_json::to_vec(value).expect("Failed to serialize weight")
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Meta {
    pub title: Option<String>,
    pub version: Option<String>,
    pub author: Option<String>,
    pub contact_information: Option<String>,
    pub reference: Option<String>,
    // pub texture: Option<u32>,
    pub allowed_user_name: Option<AllowedUserName>,
    pub violent_usage_name: Option<Allow>,
    pub sexual_usage_name: Option<Allow>,
    pub commercial_usage_name: Option<Allow>,
    pub other_permission_url: Option<String>,
    pub license_name: Option<String>,
    pub other_license_url: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Humanoid {
    // pub human_bones: Option<Vec<Bone>>,
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
pub struct Bone {
    pub bone: Option<BoneName>,
    // pub node: Option<u32>,
    pub use_default_values: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FirstPerson {
    // pub first_person_bone: Option<u32>,
    pub first_person_bone_offset: Vec3,
    pub mesh_annotations: Option<Vec<MeshAnnotation>>,
    pub look_at_type_name: Option<String>,
    pub look_at_horizontal_inner: Option<LookAtCurve>,
    pub look_at_horizontal_outer: Option<LookAtCurve>,
    pub look_at_vertical_down: Option<LookAtCurve>,
    pub look_at_vertical_up: Option<LookAtCurve>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MeshAnnotation {
    // pub mesh: Option<u32>,
    pub first_person_flag: Option<String>,
}
