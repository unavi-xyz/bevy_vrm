use bevy::{
    pbr::MaterialExtension,
    prelude::*,
    render::{
        render_asset::RenderAssets,
        render_resource::{AsBindGroup, AsBindGroupShaderType, ShaderRef, ShaderType},
    },
};

use crate::SHADER_HANDLE;

pub type MtoonMaterial = bevy::pbr::ExtendedMaterial<StandardMaterial, MtoonShader>;

#[derive(Asset, AsBindGroup, PartialEq, Debug, Clone, Component, Reflect)]
#[uniform(100, MtoonShaderUniform)]
#[reflect(PartialEq)]
pub struct MtoonShader {
    pub ambient_color: Color,
    pub gl_equalization_factor: f32,
    pub light_color: Color,
    pub light_dir: Vec3,
    pub matcap_factor: Vec4,
    pub parametric_rim_color: Color,
    pub parametric_rim_fresnel_power: f32,
    pub parametric_rim_lift_factor: f32,
    pub rim_lighting_mix_factor: f32,
    pub shade_color: Color,
    pub shading_shift_factor: f32,
    pub shading_toony_factor: f32,
    pub view_dir: Vec3,

    #[texture(101)]
    #[sampler(102)]
    #[dependency]
    pub shade_shift_texture: Option<Handle<Image>>,
    #[texture(103)]
    #[sampler(104)]
    #[dependency]
    pub shade_color_texture: Option<Handle<Image>>,
    #[texture(105)]
    #[sampler(106)]
    #[dependency]
    pub matcap_texture: Option<Handle<Image>>,
    #[texture(107)]
    #[sampler(108)]
    #[dependency]
    pub rim_multiply_texture: Option<Handle<Image>>,
}

impl Default for MtoonShader {
    fn default() -> Self {
        Self {
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

            shade_shift_texture: None,
            shade_color_texture: None,
            matcap_texture: None,
            rim_multiply_texture: None,
        }
    }
}

#[derive(Clone, Default, ShaderType)]
pub struct MtoonShaderUniform {
    pub ambient_color: Vec4,
    pub flags: u32,
    pub gl_equalization_factor: f32,
    pub light_color: Vec4,
    pub light_dir: Vec3,
    pub matcap_factor: Vec4,
    pub parametric_rim_color: Vec4,
    pub parametric_rim_fresnel_power: f32,
    pub parametric_rim_lift_factor: f32,
    pub rim_lighting_mix_factor: f32,
    pub shade_color: Vec4,
    pub shading_shift_factor: f32,
    pub shading_toony_factor: f32,
    pub view_dir: Vec3,
}

impl AsBindGroupShaderType<MtoonShaderUniform> for MtoonShader {
    fn as_bind_group_shader_type(&self, _images: &RenderAssets<Image>) -> MtoonShaderUniform {
        let mut flags = MtoonMaterialFlags::empty();

        if self.shade_shift_texture.is_some() {
            flags |= MtoonMaterialFlags::SHADING_SHIFT_TEXTURE;
        }
        if self.shade_color_texture.is_some() {
            flags |= MtoonMaterialFlags::SHADE_COLOR_TEXTURE;
        }
        if self.matcap_texture.is_some() {
            flags |= MtoonMaterialFlags::MATCAP_TEXTURE;
        }
        if self.rim_multiply_texture.is_some() {
            flags |= MtoonMaterialFlags::RIM_MULTIPLY_TEXTURE;
        }

        MtoonShaderUniform {
            ambient_color: self.ambient_color.as_linear_rgba_f32().into(),
            flags: flags.bits(),
            gl_equalization_factor: self.gl_equalization_factor,
            light_color: self.light_color.as_linear_rgba_f32().into(),
            light_dir: self.light_dir,
            matcap_factor: self.matcap_factor,
            parametric_rim_color: self.parametric_rim_color.as_linear_rgba_f32().into(),
            parametric_rim_fresnel_power: self.parametric_rim_fresnel_power,
            parametric_rim_lift_factor: self.parametric_rim_lift_factor,
            rim_lighting_mix_factor: self.rim_lighting_mix_factor,
            shade_color: self.shade_color.as_linear_rgba_f32().into(),
            shading_shift_factor: self.shading_shift_factor,
            shading_toony_factor: self.shading_toony_factor,
            view_dir: self.view_dir,
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

bitflags::bitflags! {
    #[repr(transparent)]
    pub struct MtoonMaterialFlags: u32 {
        const SHADING_SHIFT_TEXTURE = 1 << 0;
        const SHADE_COLOR_TEXTURE = 1 << 1;
        const MATCAP_TEXTURE = 1 << 2;
        const RIM_MULTIPLY_TEXTURE = 1 << 3;
    }
}
