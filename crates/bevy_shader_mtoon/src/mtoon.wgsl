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
    flags: u32,
    gl_equalization_factor: f32,
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

const MTOON_FLAGS_SHADING_SHIFT_TEXTURE: u32 = 1u;
const MTOON_FLAGS_SHADE_COLOR_TEXTURE: u32 = 2u;
const MTOON_FLAGS_MATCAP_TEXTURE: u32 = 4u;
const MTOON_FLAGS_RIM_MULTIPLY_TEXTURE: u32 = 8u;

const EPSILON: f32 = 0.00001;
const WHITE: vec3<f32> = vec3<f32>(1.0, 1.0, 1.0);

@fragment
fn fragment (
   in: VertexOutput,
   @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    var pbr_input = pbr_input_from_standard_material(in, is_front);

    // Alpha discard
    pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);

#ifdef PREPASS_PIPELINE

    let out = deferred_output(in, pbr_input);

#else

    var out: FragmentOutput;

    let base_color = pbr_input.material.base_color;

    // Shading
    var shading = dot(pbr_input.N, material.light_dir);
    shading = shading + material.shading_shift_factor;

    if (material.flags & MTOON_FLAGS_SHADING_SHIFT_TEXTURE) != 0u {
        // TODO: Convert texture sample to same type as `shading`
        // shading = shading + textureSample(shading_shift_texture, shading_shift_sampler, in.uv);
    }

    shading = 1.0 - linear_step(material.shading_toony_factor - 1.0, 1.0 - material.shading_toony_factor, shading);

    var shade_color = material.shade_color;

    if (material.flags & MTOON_FLAGS_SHADE_COLOR_TEXTURE) != 0u {
        shade_color = shade_color * textureSample(shade_color_texture, shade_color_sampler, in.uv).rgb;
    }

    var color = mix(base_color.rgb, shade_color, shading) * material.light_color;

    // Global illumination
    // This isn't really what the spec says to do, but it gives us standard Bevy lighting features,
    // such as ambient light and shadows.
    let pbr_lighting_color = apply_pbr_lighting(pbr_input);
    color += pbr_lighting_color.rgb * base_color.rgb;

    // Rim lighting
    var rim: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);

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

    rim *= mix(WHITE, pbr_lighting_color.rgb, material.rim_lighting_mix_factor);

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
