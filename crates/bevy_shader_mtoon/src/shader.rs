use bevy::{
    pbr::MaterialExtension,
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};

use crate::SHADER_HANDLE;

pub type MtoonMaterial = bevy::pbr::ExtendedMaterial<StandardMaterial, MtoonShader>;

#[derive(Asset, AsBindGroup, PartialEq, Debug, Clone, Component, Reflect)]
#[reflect(PartialEq)]
pub struct MtoonShader {
    pub base_color: Color,
    pub shade_color: Color,
    pub light_dir: Vec3,
    pub shading_shift_factor: f32,
    pub shading_toony_factor: f32,
    pub light_color: Color,

    pub ambient_color: Color,
    pub gl_equalization_factor: f32,

    pub view_dir: Vec3,
    pub matcap_factor: Vec4,
    pub parametric_rim_color: Color,
    pub parametric_rim_fresnel_power: f32,
    pub parametric_rim_lift_factor: f32,
    pub rim_lighting_mix_factor: f32,

    #[texture(101)]
    #[sampler(102)]
    pub base_color_texture: Option<Handle<Image>>,
    #[texture(103)]
    #[sampler(104)]
    pub shade_color_texture: Option<Handle<Image>>,
    #[texture(105)]
    #[sampler(106)]
    pub matcap_texture: Option<Handle<Image>>,
    #[texture(107)]
    #[sampler(108)]
    pub rim_multiply_texture: Option<Handle<Image>>,

    pub alpha_mode: AlphaMode,
}

impl Default for MtoonShader {
    fn default() -> Self {
        Self {
            base_color: Color::WHITE,
            shade_color: Color::BLACK,
            light_dir: Vec3::Y,
            shading_shift_factor: 0.0,
            shading_toony_factor: 0.9,
            light_color: Color::WHITE,

            ambient_color: Color::WHITE,
            gl_equalization_factor: 0.9,

            view_dir: Vec3::ZERO,
            matcap_factor: Vec4::ZERO,
            parametric_rim_color: Color::WHITE,
            parametric_rim_fresnel_power: 5.0,
            parametric_rim_lift_factor: 0.0,
            rim_lighting_mix_factor: 1.0,

            base_color_texture: None,
            shade_color_texture: None,
            matcap_texture: None,
            rim_multiply_texture: None,

            alpha_mode: AlphaMode::Opaque,
        }
    }
}

impl MaterialExtension for MtoonShader {
    fn fragment_shader() -> ShaderRef {
        SHADER_HANDLE.into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        SHADER_HANDLE.into()
    }
}
