//! `MToon` material from the core glTF material and its `VRMC_materials_mtoon`
//! extension.

use bevy::{
    asset::LoadContext,
    prelude::*,
};
use bevy_shader_mtoon::{
    MtoonMaterial,
    VrmOutlineMode,
};
use gltf::material::AlphaMode as GltfAlphaMode;
use serde_vrm::vrm1::vrmc_materials_mtoon::{
    OutlineWidthMode,
    VrmcMaterialsMtoon,
};

use super::texture_handle;

pub fn from_vrm1(
    load_context: &mut LoadContext,
    material: &gltf::Material,
    extension: &VrmcMaterialsMtoon,
) -> MtoonMaterial {
    let mut mtoon = MtoonMaterial::default();

    let pbr = material.pbr_metallic_roughness();
    mtoon.base_color = LinearRgba::from_f32_array(pbr.base_color_factor()).into();
    mtoon.base_color_texture = pbr
        .base_color_texture()
        .map(|info| texture_handle(load_context, info.texture().index()));

    mtoon.alpha_mode = match material.alpha_mode() {
        GltfAlphaMode::Opaque => AlphaMode::Opaque,
        GltfAlphaMode::Mask => AlphaMode::Mask(material.alpha_cutoff().unwrap_or(0.5)),
        GltfAlphaMode::Blend => AlphaMode::Blend,
    };
    mtoon.double_sided = material.double_sided();

    if let Some(normal) = material.normal_texture() {
        mtoon.normal_map_scale = normal.scale();
        mtoon.normal_map_texture = Some(texture_handle(load_context, normal.texture().index()));
    }

    let strength = material.emissive_strength().unwrap_or(1.0);
    let [r, g, b] = material.emissive_factor();
    mtoon.emissive_factor = LinearRgba::new(r * strength, g * strength, b * strength, 1.0).into();
    mtoon.emissive_texture = material
        .emissive_texture()
        .map(|info| texture_handle(load_context, info.texture().index()));

    mtoon.shade_factor = rgb(extension.shade_color_factor);
    mtoon.shade_multiply_texture = extension
        .shade_multiply_texture
        .map(|t| texture_handle(load_context, t.index as usize));
    mtoon.shading_shift_factor = extension.shading_shift_factor;
    mtoon.shade_shift_texture = extension
        .shading_shift_texture
        .map(|t| texture_handle(load_context, t.index as usize));
    mtoon.shading_toony_factor = extension.shading_toony_factor;
    mtoon.gi_equalization_factor = extension.gi_equalization_factor;

    mtoon.matcap_factor = Vec3::from_array(extension.matcap_factor);
    mtoon.matcap_texture = extension
        .matcap_texture
        .map(|t| texture_handle(load_context, t.index as usize));

    mtoon.parametric_rim_color = rgb(extension.parametric_rim_color_factor);
    mtoon.parametric_rim_fresnel_power = extension.parametric_rim_fresnel_power_factor;
    mtoon.parametric_rim_lift_factor = extension.parametric_rim_lift_factor;
    mtoon.rim_lighting_mix_factor = extension.rim_lighting_mix_factor;
    mtoon.rim_multiply_texture = extension
        .rim_multiply_texture
        .map(|t| texture_handle(load_context, t.index as usize));

    mtoon.outline_mode = match extension.outline_width_mode {
        OutlineWidthMode::None => VrmOutlineMode::None,
        OutlineWidthMode::WorldCoordinates => VrmOutlineMode::World,
        OutlineWidthMode::ScreenCoordinates => VrmOutlineMode::Screen,
    };
    mtoon.outline_width = extension.outline_width_factor;
    mtoon.outline_color = rgb(extension.outline_color_factor);

    mtoon
}

fn rgb([r, g, b]: [f32; 3]) -> Color {
    LinearRgba::new(r, g, b, 1.0).into()
}
