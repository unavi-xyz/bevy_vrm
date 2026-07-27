use bevy::{
    asset::LoadContext,
    gltf::GltfAssetLabel,
    image::Image,
    prelude::*,
};
use bevy_shader_mtoon::{
    MtoonMaterial,
    VrmOutlineMode,
};
use serde_vrm::vrm0::MaterialProperty;

fn texture_handle(load_context: &mut LoadContext, index: u32) -> Handle<Image> {
    load_context.get_label_handle(GltfAssetLabel::Texture(index as usize).to_string())
}

pub fn build_mtoon_material(
    load_context: &mut LoadContext,
    props: &MaterialProperty,
) -> MtoonMaterial {
    let mut mtoon = MtoonMaterial::default();

    let float = props.float.as_ref();
    let vector = props.vector.as_ref();
    let texture = props.texture.as_ref();
    let keyword = props.keyword_map.as_ref();

    if let Some(value) = float.and_then(|f| f.double_sided) {
        mtoon.double_sided = value == 0.0;
    }

    if let Some(value) = float.and_then(|f| f.cutoff) {
        mtoon.alpha_mode = AlphaMode::Mask(value);
    }

    if let Some(value) = vector.and_then(|v| v.color) {
        mtoon.base_color = LinearRgba::from_f32_array(value).into();
    }

    mtoon.base_color_texture = texture
        .and_then(|t| t.base_color)
        .map(|i| texture_handle(load_context, i));

    if let Some(value) = float.and_then(|f| f.normal_scale) {
        mtoon.normal_map_scale = value;
    }

    mtoon.normal_map_texture = texture
        .and_then(|t| t.normal)
        .map(|i| texture_handle(load_context, i));

    if let Some(value) = vector.and_then(|v| v.emissive_factor) {
        mtoon.emissive_factor = LinearRgba::from_f32_array(value).into();
    }

    mtoon.emissive_texture = texture
        .and_then(|t| t.emissive)
        .map(|i| texture_handle(load_context, i));

    if let Some(value) = float.and_then(|f| f.outline_factor) {
        mtoon.outline_width = value;
    }

    if let Some(value) = vector.and_then(|v| v.outline_color) {
        mtoon.outline_color = LinearRgba::from_f32_array(value).into();
    }

    if let Some(value) = keyword.and_then(|k| k.outline_width_world) {
        mtoon.outline_mode = if value {
            VrmOutlineMode::World
        } else {
            VrmOutlineMode::Screen
        };
    }

    if let Some(value) = float.and_then(|f| f.gi_intensity_factor) {
        mtoon.gi_equalization_factor = 1.0 - value;
    }

    if let Some(value) = float.and_then(|f| f.shade_shift) {
        mtoon.shading_shift_factor = -value;
    }

    if let Some(value) = float.and_then(|f| f.shade_toony) {
        mtoon.shading_toony_factor = value;
    }

    if let Some(value) = vector.and_then(|v| v.shade_color) {
        mtoon.shade_factor = LinearRgba::from_f32_array(value).into();
    }

    mtoon.shade_multiply_texture = texture
        .and_then(|t| t.shade)
        .map(|i| texture_handle(load_context, i));

    mtoon
}
