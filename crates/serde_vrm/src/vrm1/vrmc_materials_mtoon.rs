//! The `VRMC_materials_mtoon` extension.

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VrmcMaterialsMtoon {
    pub spec_version:                        String,
    #[serde(default)]
    pub transparent_with_z_write:            bool,
    #[serde(default)]
    pub render_queue_offset_number:          i32,
    /// Defaults to black per the spec README, which overrides the schema JSON.
    #[serde(default)]
    pub shade_color_factor:                  [f32; 3],
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shade_multiply_texture:              Option<TextureInfo>,
    #[serde(default)]
    pub shading_shift_factor:                f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shading_shift_texture:               Option<ShadingShiftTexture>,
    #[serde(default = "default_toony")]
    pub shading_toony_factor:                f32,
    #[serde(default = "default_toony")]
    pub gi_equalization_factor:              f32,
    #[serde(default = "one3")]
    pub matcap_factor:                       [f32; 3],
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matcap_texture:                      Option<TextureInfo>,
    #[serde(default)]
    pub parametric_rim_color_factor:         [f32; 3],
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rim_multiply_texture:                Option<TextureInfo>,
    #[serde(default = "one")]
    pub rim_lighting_mix_factor:             f32,
    #[serde(default = "default_fresnel_power")]
    pub parametric_rim_fresnel_power_factor: f32,
    #[serde(default)]
    pub parametric_rim_lift_factor:          f32,
    #[serde(default)]
    pub outline_width_mode:                  OutlineWidthMode,
    #[serde(default)]
    pub outline_width_factor:                f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outline_width_multiply_texture:      Option<TextureInfo>,
    #[serde(default)]
    pub outline_color_factor:                [f32; 3],
    #[serde(default = "one")]
    pub outline_lighting_mix_factor:         f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uv_animation_mask_texture:           Option<TextureInfo>,
    #[serde(default)]
    pub uv_animation_scroll_x_speed_factor:  f32,
    #[serde(default)]
    pub uv_animation_scroll_y_speed_factor:  f32,
    #[serde(default)]
    pub uv_animation_rotation_speed_factor:  f32,
}

impl VrmcMaterialsMtoon {
    pub const EXTENSION_NAME: &str = "VRMC_materials_mtoon";
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum OutlineWidthMode {
    #[default]
    None,
    WorldCoordinates,
    ScreenCoordinates,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextureInfo {
    pub index:     u32,
    #[serde(default)]
    pub tex_coord: u32,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShadingShiftTexture {
    pub index:     u32,
    #[serde(default)]
    pub tex_coord: u32,
    #[serde(default = "one")]
    pub scale:     f32,
}

const fn one() -> f32 {
    1.0
}

const fn one3() -> [f32; 3] {
    [1.0; 3]
}

const fn default_toony() -> f32 {
    0.9
}

const fn default_fresnel_power() -> f32 {
    5.0
}
