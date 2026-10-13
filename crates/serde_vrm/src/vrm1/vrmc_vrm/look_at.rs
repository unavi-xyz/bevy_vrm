//! Eye gaze control.

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LookAt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_from_head_bone:      Option<[f32; 3]>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind:                       Option<LookAtType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_map_horizontal_inner: Option<RangeMap>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_map_horizontal_outer: Option<RangeMap>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_map_vertical_down:    Option<RangeMap>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_map_vertical_up:      Option<RangeMap>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum LookAtType {
    Bone,
    Expression,
}

/// Input is an angle in degrees; output is degrees for `bone`, weight for
/// `expression`.
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RangeMap {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_max_value: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_scale:    Option<f32>,
}
