use bevy::{
    asset::load_internal_asset,
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, AsBindGroupShaderType, ShaderRef, ShaderType},
};

pub const MTOON_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(0x2d86c40a175b);

#[derive(Default)]
pub struct MtoonPlugin;

impl Plugin for MtoonPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(app, MTOON_SHADER_HANDLE, "mtoon.wgsl", Shader::from_wgsl);

        app.add_plugins(MaterialPlugin::<MtoonMaterial>::default())
            .add_systems(Update, update_mtoon_shader);
    }
}

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
#[uniform(0, MtoonMaterialUniform)]
pub struct MtoonMaterial {
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

    #[texture(1)]
    #[sampler(2)]
    pub base_color_texture: Option<Handle<Image>>,
    #[texture(3)]
    #[sampler(4)]
    pub shade_color_texture: Option<Handle<Image>>,
    #[texture(5)]
    #[sampler(6)]
    pub matcap_texture: Option<Handle<Image>>,
    #[texture(7)]
    #[sampler(8)]
    pub rim_multiply_texture: Option<Handle<Image>>,
}

impl Default for MtoonMaterial {
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
        }
    }
}

impl Material for MtoonMaterial {
    fn fragment_shader() -> ShaderRef {
        MTOON_SHADER_HANDLE.into()
    }
}

#[derive(Clone, Default, ShaderType)]
pub struct MtoonMaterialUniform {
    pub base_color: Vec4,
    pub shade_color: Vec4,
    pub light_dir: Vec3,
    pub shading_shift_factor: f32,
    pub shading_toony_factor: f32,
    pub light_color: Vec4,

    pub ambient_color: Vec4,
    pub gl_equalization_factor: f32,

    pub view_dir: Vec3,
    pub matcap_factor: Vec4,
    pub parametric_rim_color: Vec4,
    pub parametric_rim_fresnel_power: f32,
    pub parametric_rim_lift_factor: f32,
    pub rim_lighting_mix_factor: f32,
}

impl AsBindGroupShaderType<MtoonMaterialUniform> for MtoonMaterial {
    fn as_bind_group_shader_type(
        &self,
        _images: &bevy::render::render_asset::RenderAssets<Image>,
    ) -> MtoonMaterialUniform {
        MtoonMaterialUniform {
            base_color: Color::rgba_to_vec4(&self.base_color),
            shade_color: Color::rgba_to_vec4(&self.shade_color),
            light_dir: self.light_dir,
            shading_shift_factor: self.shading_shift_factor,
            shading_toony_factor: self.shading_toony_factor,
            light_color: Color::rgba_to_vec4(&self.light_color),

            ambient_color: Color::rgba_to_vec4(&self.ambient_color),
            gl_equalization_factor: self.gl_equalization_factor,

            view_dir: self.view_dir,
            matcap_factor: self.matcap_factor,
            parametric_rim_color: Color::rgba_to_vec4(&self.parametric_rim_color),
            parametric_rim_fresnel_power: self.parametric_rim_fresnel_power,
            parametric_rim_lift_factor: self.parametric_rim_lift_factor,
            rim_lighting_mix_factor: self.rim_lighting_mix_factor,
        }
    }
}

#[derive(Component)]
pub struct MtoonMainCamera;

#[derive(Component)]
pub struct MtoonSun;

pub fn update_mtoon_shader(
    main_cam: Query<&Transform, With<MtoonMainCamera>>,
    sun: Query<(&Transform, &DirectionalLight), With<MtoonSun>>,
    ambient_light: Option<Res<AmbientLight>>,
    mut materials: ResMut<Assets<MtoonMaterial>>,
) {
    for (_, mtoon) in materials.iter_mut() {
        if let Ok(cam_t) = main_cam.get_single() {
            mtoon.view_dir = *cam_t.back();
        }

        if let Ok((transform, light)) = sun.get_single() {
            mtoon.light_dir = *transform.back();
            mtoon.light_color = light.color;
        }

        if let Some(light) = &ambient_light {
            let mut ambient_color = light.color;
            ambient_color.set_a(ambient_color.a() * light.brightness);
            mtoon.ambient_color = ambient_color;
        }
    }
}
