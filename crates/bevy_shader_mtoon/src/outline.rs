use bevy::{
    prelude::*,
    render::render_resource::{
        AsBindGroup,
        Face,
        ShaderType,
    },
    shader::ShaderRef,
};

use crate::{
    MtoonMaterial,
    OUTLINE_SHADER_HANDLE,
    shader::OutlineMode,
};

#[derive(Asset, AsBindGroup, Clone, Debug, PartialEq, Reflect)]
#[uniform(0, OutlineShaderUniform)]
pub struct OutlineMaterial {
    pub color: Color,
    pub width: f32,
    pub mode:  OutlineMode,
}

impl OutlineMaterial {
    pub(crate) const fn from_mtoon(mtoon: &MtoonMaterial) -> Self {
        Self {
            color: mtoon.outline_color,
            width: mtoon.outline_width,
            mode:  mtoon.outline_mode,
        }
    }
}

#[derive(Clone, Default, ShaderType)]
pub struct OutlineShaderUniform {
    pub color: Vec4,
    pub width: f32,
    pub mode:  u32,
}

impl bevy::render::render_resource::AsBindGroupShaderType<OutlineShaderUniform>
    for OutlineMaterial
{
    fn as_bind_group_shader_type(
        &self,
        _images: &bevy::render::render_asset::RenderAssets<bevy::render::texture::GpuImage>,
    ) -> OutlineShaderUniform {
        OutlineShaderUniform {
            color: self.color.to_linear().to_f32_array().into(),
            width: self.width,
            mode:  match self.mode {
                OutlineMode::None => 0,
                OutlineMode::World => 1,
                OutlineMode::Screen => 2,
            },
        }
    }
}

impl Material for OutlineMaterial {
    fn vertex_shader() -> ShaderRef {
        OUTLINE_SHADER_HANDLE.into()
    }

    fn fragment_shader() -> ShaderRef {
        OUTLINE_SHADER_HANDLE.into()
    }

    fn specialize(
        _pipeline: &bevy::pbr::MaterialPipeline,
        descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
        _layout: &bevy::mesh::MeshVertexBufferLayoutRef,
        _key: bevy::pbr::MaterialPipelineKey<Self>,
    ) -> Result<(), bevy::render::render_resource::SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = Some(Face::Front);
        Ok(())
    }
}
