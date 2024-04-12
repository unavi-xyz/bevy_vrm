#import bevy_pbr::{
    pbr_fragment::pbr_input_from_vertex_output,
    mesh_view_bindings::view,
    ambient::ambient_light,
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
    base_color: vec4<f32>,
    emissive_factor: vec4<f32>,
    flags: u32,
    gi_equalization_factor: f32,
    light_color: vec3<f32>,
    light_dir: vec3<f32>,
    matcap_factor: vec3<f32>,
    parametric_rim_color: vec3<f32>,
    parametric_rim_fresnel_power: f32, 
    parametric_rim_lift_factor: f32,
    rim_lighting_mix_factor: f32,
    shade_color: vec3<f32>,
    shading_shift_factor: f32,
    shading_toony_factor: f32,
};

@group(2) @binding(0)
var<uniform> material: MtoonMaterialUniform;

@group(2) @binding(1) var base_color_texture: texture_2d<f32>;
@group(2) @binding(2) var base_color_sampler: sampler;
@group(2) @binding(3) var emissive_texture: texture_2d<f32>;
@group(2) @binding(4) var emissive_sampler: sampler;
@group(2) @binding(5) var matcap_texture: texture_2d<f32>;
@group(2) @binding(6) var matcap_sampler: sampler;
@group(2) @binding(7) var normal_map_texture: texture_2d<f32>;
@group(2) @binding(8) var normal_map_sampler: sampler;
@group(2) @binding(9) var rim_multiply_texture: texture_2d<f32>;
@group(2) @binding(10) var rim_multiply_sampler: sampler;
@group(2) @binding(11) var shade_color_texture: texture_2d<f32>;
@group(2) @binding(12) var shade_color_sampler: sampler;
@group(2) @binding(13) var shade_shift_texture: texture_2d<f32>;
@group(2) @binding(14) var shade_shift_sampler: sampler;

const MTOON_FLAGS_BASE_COLOR_TEXTURE: u32 = 1u;
const MTOON_FLAGS_EMISSIVE_TEXTURE: u32 = 2u;
const MTOON_FLAGS_MATCAP_TEXTURE: u32 = 4u;
const MTOON_FLAGS_RIM_MULTIPLY_TEXTURE: u32 = 8u;
const MTOON_FLAGS_SHADE_COLOR_TEXTURE: u32 = 16u;
const MTOON_FLAGS_SHADING_SHIFT_TEXTURE: u32 = 32u;

const EPSILON: f32 = 0.00001;

@fragment
fn fragment (
   in: VertexOutput,
   @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    let double_sided = true;
    var pbr_input = pbr_input_from_vertex_output(in, is_front, double_sided);

#ifdef PREPASS_PIPELINE

    let out = deferred_output(in, pbr_input);

#else

    var out: FragmentOutput;

    // Base color
    pbr_input.material.base_color = material.base_color;

    if (material.flags & MTOON_FLAGS_BASE_COLOR_TEXTURE) != 0u {
        pbr_input.material.base_color *= textureSample(base_color_texture, base_color_sampler, in.uv);
    }

    let base_color = pbr_input.material.base_color;

    // Emissive
    var emissive = material.emissive_factor;

    if ((material.flags & MTOON_FLAGS_EMISSIVE_TEXTURE) != 0u) {
      emissive = vec4<f32>(emissive.rgb * textureSampleBias(emissive_texture, emissive_sampler, in.uv, view.mip_bias).rgb, 1.0);
    }


    // Normal

    // Shading
    var shading = dot(pbr_input.N, material.light_dir);
    shading = shading + material.shading_shift_factor;

    if (material.flags & MTOON_FLAGS_SHADING_SHIFT_TEXTURE) != 0u {
        // TODO: Convert texture sample to same type as `shading`
        // shading = shading + textureSample(shade_shift_texture, shade_shift_sampler, in.uv);
    }

    shading = 1.0 - linear_step(material.shading_toony_factor - 1.0, 1.0 - material.shading_toony_factor, shading);

    var shade_color = material.shade_color;

    if (material.flags & MTOON_FLAGS_SHADE_COLOR_TEXTURE) != 0u {
        shade_color *= textureSample(shade_color_texture, shade_color_sampler, in.uv).rgb;
    }

    var color = mix(base_color.rgb, shade_color, shading);
    color *= material.light_color;

    // Global illumination
    pbr_input.material.base_color = vec4<f32>(color, base_color.a);
    let pbr_lighting_color = apply_pbr_lighting(pbr_input);
    
    let n_dot_v = max(dot(pbr_input.N, pbr_input.V), 0.0001);
    let diffuse_color = base_color.rgb;
    let F0 = base_color.rgb;
    let perceptual_roughness = pbr_input.material.perceptual_roughness;
    let diffuse_occlusion = pbr_input.diffuse_occlusion;
    var uniform_gi = ambient_light(pbr_input.world_position, pbr_input.N, pbr_input.V, n_dot_v, diffuse_color, F0, perceptual_roughness, diffuse_occlusion);

    uniform_gi *= base_color.rgb;
    uniform_gi *= view.exposure;

    let gi = mix(pbr_lighting_color.rgb, uniform_gi, material.gi_equalization_factor);
    color += gi;

    // Rim lighting
    var rim = vec3(0.0);

    if (material.flags & MTOON_FLAGS_MATCAP_TEXTURE) != 0u {
        let world_view_x = normalize(vec3<f32>(pbr_input.V.z, 0.0, -pbr_input.V.x));
        let world_view_y = cross(pbr_input.V, world_view_x);
        let matcap_uv = vec2<f32>(dot(world_view_x, pbr_input.N), dot(world_view_y, pbr_input.N)) * 0.495 + 0.5;
        let matcap_color = textureSample(matcap_texture, matcap_sampler, matcap_uv);
        rim = material.matcap_factor * matcap_color.rgb;
    }

    var parametric_rim = saturate(1.0 - dot(pbr_input.N, pbr_input.V) + material.parametric_rim_lift_factor);
    parametric_rim = pow(parametric_rim, max(material.parametric_rim_fresnel_power, EPSILON));

    rim += parametric_rim * material.parametric_rim_color;

    if (material.flags & MTOON_FLAGS_RIM_MULTIPLY_TEXTURE) != 0u {
        let rim_multiply = textureSample(rim_multiply_texture, rim_multiply_sampler, in.uv);
        rim *= rim_multiply.rgb;
    }

    rim *= mix(vec3(1.0), pbr_lighting_color.rgb, material.rim_lighting_mix_factor);

    color += rim;

    // Set output
    out.color = vec4<f32>(color, base_color.a);
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);

#endif

    return out;
}

fn linear_step(a: f32, b: f32, t: f32) -> f32 {
    return saturate((t - a) / (b - a));
}
