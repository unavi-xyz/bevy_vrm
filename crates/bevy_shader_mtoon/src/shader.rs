use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssets,
        render_resource::{AsBindGroup, AsBindGroupShaderType, Face, ShaderRef, ShaderType},
        texture::GpuImage,
    },
};

use crate::SHADER_HANDLE;

#[derive(Asset, AsBindGroup, PartialEq, Debug, Clone, Component, Reflect)]
#[bind_group_data(MtoonMaterialKey)]
#[uniform(0, MtoonShaderUniform)]
#[reflect(PartialEq)]
pub struct MtoonMaterial {
    pub outline_color: Color,
    pub outline_mode: OutlineMode,
    pub outline_width: f32,

    pub alpha_mode: AlphaMode,
    pub base_color: Color,
    pub double_sided: bool,
    pub emissive_factor: Color,
    pub gi_equalization_factor: f32,
    pub light_color: Color,
    pub light_dir: Vec3,
    pub matcap_factor: Vec3,
    pub normal_map_scale: f32,
    pub parametric_rim_color: Color,
    pub parametric_rim_fresnel_power: f32,
    pub parametric_rim_lift_factor: f32,
    pub rim_lighting_mix_factor: f32,
    pub shade_factor: Color,
    pub shading_shift_factor: f32,
    pub shading_toony_factor: f32,
    pub view_dir: Vec3,

    #[texture(1)]
    #[sampler(2)]
    #[dependency]
    pub base_color_texture: Option<Handle<Image>>,
    #[texture(3)]
    #[sampler(4)]
    #[dependency]
    pub emissive_texture: Option<Handle<Image>>,
    #[texture(5)]
    #[sampler(6)]
    #[dependency]
    pub matcap_texture: Option<Handle<Image>>,
    #[texture(7)]
    #[sampler(8)]
    #[dependency]
    pub normal_map_texture: Option<Handle<Image>>,
    #[texture(9)]
    #[sampler(10)]
    #[dependency]
    pub rim_multiply_texture: Option<Handle<Image>>,
    #[texture(11)]
    #[sampler(12)]
    #[dependency]
    pub shade_multiply_texture: Option<Handle<Image>>,
    #[texture(13)]
    #[sampler(14)]
    #[dependency]
    pub shade_shift_texture: Option<Handle<Image>>,
}

#[derive(Debug, Default, Clone, PartialEq, Reflect)]
pub enum OutlineMode {
    #[default]
    None,
    Screen,
    World,
}

impl Default for MtoonMaterial {
    fn default() -> Self {
        Self {
            outline_color: Color::BLACK,
            outline_mode: OutlineMode::None,
            outline_width: 0.0,

            alpha_mode: AlphaMode::Opaque,
            base_color: Color::WHITE,
            double_sided: false,
            emissive_factor: Color::BLACK,
            gi_equalization_factor: 0.9,
            light_color: Color::WHITE,
            light_dir: Vec3::Y,
            matcap_factor: Vec3::ZERO,
            normal_map_scale: 1.0,
            parametric_rim_color: Color::WHITE,
            parametric_rim_fresnel_power: 5.0,
            parametric_rim_lift_factor: 0.0,
            rim_lighting_mix_factor: 1.0,
            shade_factor: Color::BLACK,
            shading_shift_factor: 0.0,
            shading_toony_factor: 0.9,
            view_dir: Vec3::ZERO,

            base_color_texture: None,
            emissive_texture: None,
            matcap_texture: None,
            normal_map_texture: None,
            rim_multiply_texture: None,
            shade_multiply_texture: None,
            shade_shift_texture: None,
        }
    }
}

#[derive(Clone, Default, ShaderType)]
pub struct MtoonShaderUniform {
    pub alpha_cutoff: f32,
    pub base_color: Vec4,
    pub emissive_factor: Vec4,
    pub flags: u32,
    pub gi_equalization_factor: f32,
    pub light_color: Vec3,
    pub light_dir: Vec3,
    pub matcap_factor: Vec3,
    pub normal_map_scale: f32,
    pub parametric_rim_color: Vec3,
    pub parametric_rim_fresnel_power: f32,
    pub parametric_rim_lift_factor: f32,
    pub rim_lighting_mix_factor: f32,
    pub shade_color: Vec3,
    pub shading_shift_factor: f32,
    pub shading_toony_factor: f32,
    pub view_dir: Vec3,
}

impl AsBindGroupShaderType<MtoonShaderUniform> for MtoonMaterial {
    fn as_bind_group_shader_type(&self, _images: &RenderAssets<GpuImage>) -> MtoonShaderUniform {
        let mut flags = MtoonMaterialFlags::empty();

        if self.base_color_texture.is_some() {
            flags |= MtoonMaterialFlags::BASE_COLOR_TEXTURE;
        }
        if self.emissive_texture.is_some() {
            flags |= MtoonMaterialFlags::EMISSIVE_TEXTURE;
        }
        if self.matcap_texture.is_some() {
            flags |= MtoonMaterialFlags::MATCAP_TEXTURE;
        }
        if self.normal_map_texture.is_some() {
            flags |= MtoonMaterialFlags::NORMAL_MAP_TEXTURE
        }
        if self.rim_multiply_texture.is_some() {
            flags |= MtoonMaterialFlags::RIM_MULTIPLY_TEXTURE;
        }
        if self.shade_multiply_texture.is_some() {
            flags |= MtoonMaterialFlags::SHADE_COLOR_TEXTURE;
        }
        if self.shade_shift_texture.is_some() {
            flags |= MtoonMaterialFlags::SHADING_SHIFT_TEXTURE;
        }

        let alpha_cutoff = match self.alpha_mode {
            AlphaMode::Mask(value) => {
                flags |= MtoonMaterialFlags::ALPHA_MODE_MASK;
                value
            }
            AlphaMode::Opaque => {
                flags |= MtoonMaterialFlags::ALPHA_MODE_OPAQUE;
                0.0
            }
            _ => 0.0,
        };

        let light_color = self.light_color.to_linear().to_f32_array();
        let light_color = Vec3::new(light_color[0], light_color[1], light_color[2]);

        let parametric_rim_color = self.parametric_rim_color.to_linear().to_f32_array();
        let parametric_rim_color = Vec3::new(
            parametric_rim_color[0],
            parametric_rim_color[1],
            parametric_rim_color[2],
        );

        let shade_color = self.shade_factor.to_linear().to_f32_array();
        let shade_color = Vec3::new(shade_color[0], shade_color[1], shade_color[2]);

        MtoonShaderUniform {
            alpha_cutoff,
            base_color: self.base_color.to_linear().to_f32_array().into(),
            emissive_factor: self.emissive_factor.to_linear().to_f32_array().into(),
            flags: flags.bits(),
            gi_equalization_factor: self.gi_equalization_factor,
            light_color,
            light_dir: self.light_dir,
            matcap_factor: self.matcap_factor,
            normal_map_scale: self.normal_map_scale,
            parametric_rim_color,
            parametric_rim_fresnel_power: self.parametric_rim_fresnel_power,
            parametric_rim_lift_factor: self.parametric_rim_lift_factor,
            rim_lighting_mix_factor: self.rim_lighting_mix_factor,
            shade_color,
            shading_shift_factor: self.shading_shift_factor,
            shading_toony_factor: self.shading_toony_factor,
            view_dir: self.view_dir,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MtoonMaterialKey {
    cull_mode: Option<Face>,
}

impl From<&MtoonMaterial> for MtoonMaterialKey {
    fn from(material: &MtoonMaterial) -> Self {
        MtoonMaterialKey {
            cull_mode: if material.double_sided {
                None
            } else {
                Some(Face::Back)
            },
        }
    }
}

impl Material for MtoonMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_HANDLE.into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        SHADER_HANDLE.into()
    }

    fn specialize(
        _pipeline: &bevy::pbr::MaterialPipeline<Self>,
        descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
        _layout: &bevy::render::mesh::MeshVertexBufferLayoutRef,
        key: bevy::pbr::MaterialPipelineKey<Self>,
    ) -> Result<(), bevy::render::render_resource::SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = key.bind_group_data.cull_mode;
        Ok(())
    }
}

bitflags::bitflags! {
    #[repr(transparent)]
    pub struct MtoonMaterialFlags: u32 {
        const ALPHA_MODE_MASK= 1 << 0;
        const ALPHA_MODE_OPAQUE = 1 << 1;
        const BASE_COLOR_TEXTURE = 1 << 2;
        const DOUBLE_SIDED = 1 << 3;
        const EMISSIVE_TEXTURE = 1 << 4;
        const MATCAP_TEXTURE = 1 << 5;
        const NORMAL_MAP_TEXTURE = 1 << 6;
        const RIM_MULTIPLY_TEXTURE = 1 << 7;
        const SHADE_COLOR_TEXTURE = 1 << 8;
        const SHADING_SHIFT_TEXTURE = 1 << 9;
    }
}
