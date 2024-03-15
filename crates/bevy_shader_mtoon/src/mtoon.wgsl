#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
    mesh_view_bindings as view_bindings,
}

#ifdef PREPASS_PIPELINE
#import bevy_pbr::{
    prepass_io::{VertexOutput, FragmentOutput},
    pbr_deferred_functions::deferred_output,
}
#else
#import bevy_pbr::{
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
}
#endif

struct MtoonMaterialUniform {
    ambient_color: vec4<f32>,
    base_color: vec4<f32>,
    flags: u32,
    gl_equalization_factor: f32,
    light_color: vec4<f32>,
    light_dir: vec3<f32>,
    matcap_factor: vec4<f32>,
    parametric_rim_color: vec4<f32>,
    parametric_rim_fresnel_power: f32, 
    parametric_rim_lift_factor: f32,
    rim_lighting_mix_factor: f32,
    shade_color: vec4<f32>,
    shading_shift_factor: f32,
    shading_toony_factor: f32,
    view_dir: vec3<f32>,
};

@group(2) @binding(100)
var<uniform> material: MtoonMaterialUniform;

@group(2) @binding(101) var shading_shift_texture: texture_2d<f32>;
@group(2) @binding(102) var shading_shift_sampler: sampler;
@group(2) @binding(103) var shade_color_texture: texture_2d<f32>;
@group(2) @binding(104) var shade_color_sampler: sampler;
@group(2) @binding(105) var matcap_texture: texture_2d<f32>;
@group(2) @binding(106) var matcap_sampler: sampler;
@group(2) @binding(107) var rim_multiply_texture: texture_2d<f32>;
@group(2) @binding(108) var rim_multiply_sampler: sampler;

const MTOON_FLAGS_SHADE_SHIFT_TEXTURE: u32 = 1u;
const MTOON_FLAGS_SHADE_COLOR_TEXTURE: u32 = 2u;
const MTOON_FLAGS_MATCAP_TEXTURE: u32 = 4u;
const MTOON_FLAGS_RIM_MULTIPLY_TEXTURE: u32 = 8u;

@fragment
fn fragment (
   in: VertexOutput,
   @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    var pbr_input = pbr_input_from_standard_material(in, is_front);

    // Discard alpha
    pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);

#ifdef PREPASS_PIPELINE

    let out = deferred_output(in, pbr_input);

#else

    // Remove texture
    let base_color = pbr_input.material.base_color;
    pbr_input.material.base_color = vec4<f32>(1.0, 1.0, 1.0, 1.0);

    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr_input);

    // Shading
    var shade_color = material.shade_color;

    if (material.flags & MTOON_FLAGS_SHADE_COLOR_TEXTURE) != 0u {
        shade_color = shade_color * textureSample(shade_color_texture, shade_color_sampler, in.uv);
    }

    let normal = normalize(in.world_normal);

    var shading = dot(normal, material.light_dir);
    shading = shading + material.shading_shift_factor;

    if (material.flags & MTOON_FLAGS_SHADE_SHIFT_TEXTURE) != 0u {
        // TODO: Convert texture sample to same type as `shading`
        // shading = shading + textureSample(shading_shift_texture, shading_shift_sampler, in.uv);
    }

    shading = 1.0 - linear_step(material.shading_toony_factor - 1.0, 1.0 - material.shading_toony_factor, shading);

    // TODO: Rim lighting

    // Re-apply texture
    out.color = out.color * base_color;
    pbr_input.material.base_color = base_color;

    out.color = main_pass_post_lighting_processing(pbr_input, out.color);

#endif

    return out;
}

fn linear_step(a: f32, b: f32, t: f32) -> f32 {
    return saturate((t - a) / (b - a));
}
