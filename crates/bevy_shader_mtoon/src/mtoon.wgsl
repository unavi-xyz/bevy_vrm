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

struct MtoonShader {
    base_color: vec4<f32>,
    shade_color: vec4<f32>,
    light_dir: vec3<f32>,
    shading_shift_factor: f32,
    shading_toony_factor: f32,
    light_color: vec4<f32>,

    ambient_color: vec4<f32>,
    gl_equalization_factor: f32,

    view_dir: vec3<f32>,
    matcap_factor: vec4<f32>,
    parametric_rim_color: vec4<f32>,
    parametric_rim_fresnel_power: f32, 
    parametric_rim_lift_factor: f32,
    rim_lighting_mix_factor: f32,
};

@group(2) @binding(1) var base_color_texture: texture_2d<f32>;
@group(2) @binding(2) var base_color_sampler: sampler;
@group(2) @binding(3) var shade_color_texture: texture_2d<f32>;
@group(2) @binding(4) var shade_color_sampler: sampler;
//var shading_shift_texture: texture_2d<f32>;
//var shading_shift_sampler: sampler;
//var normal_texture: texture_2d<f32>;
//var normal_sampler: sampler;
//var emissive_texture: texture_2d<f32>;
//var emissive_sampler: sampler;
@group(2) @binding(5) var matcap_texture: texture_2d<f32>;
@group(2) @binding(6) var matcap_sampler: sampler;
@group(2) @binding(7) var rim_multiply_texture: texture_2d<f32>;
@group(2) @binding(8) var rim_multiply_sampler: sampler;

const RIM_EPSILON = 0.00001;
const VEC4_ONE = vec4<f32>(1.0, 1.0, 1.0, 1.0);

@fragment
fn fragment (
   in: VertexOutput,
   @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    var pbr_input = pbr_input_from_standard_material(in, is_front);

#ifdef PREPASS_PIPELINE

    let out = deferred_output(in, pbr_input);

#else

    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr_input);

#endif

    return out;
}

fn linear_step(a: f32, b: f32, t: f32) -> f32 {
    return saturate((t - a) / (b - a));
}
