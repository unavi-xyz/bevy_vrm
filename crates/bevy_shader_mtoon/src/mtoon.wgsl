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
    alpha_cutoff: f32,
    base_color: vec4<f32>,
    emissive_factor: vec4<f32>,
    flags: u32,
    gi_equalization_factor: f32,
    light_color: vec3<f32>,
    light_dir: vec3<f32>,
    matcap_factor: vec3<f32>,
    normal_map_scale: f32,
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

const MTOON_FLAGS_ALPHA_MODE_MASK: u32 = 1u;
const MTOON_FLAGS_ALPHA_MODE_OPAQUE: u32 = 2u;
const MTOON_FLAGS_BASE_COLOR_TEXTURE: u32 = 4u;
const MTOON_FLAGS_DOUBLE_SIDED: u32 = 8u;
const MTOON_FLAGS_EMISSIVE_TEXTURE: u32 = 16u;
const MTOON_FLAGS_MATCAP_TEXTURE: u32 = 32u;
const MTOON_FLAGS_NORMAL_MAP_TEXTURE: u32 = 64u;
const MTOON_FLAGS_RIM_MULTIPLY_TEXTURE: u32 = 128u;
const MTOON_FLAGS_SHADE_COLOR_TEXTURE: u32 = 256u;
const MTOON_FLAGS_SHADING_SHIFT_TEXTURE: u32 = 512u;

const EPSILON: f32 = 0.00001;

@fragment
fn fragment (
   in: VertexOutput,
   @builtin(front_facing) is_front: bool,
) -> @location(0) vec4<f32> {
    let double_sided = (material.flags & MTOON_FLAGS_DOUBLE_SIDED) != 0;
    var pbr_input = pbr_input_from_vertex_output(in, is_front, double_sided);

    pbr_input.material.metallic = 0.0;
    pbr_input.material.perceptual_roughness = 0.9;

    // Base color.
    var base_color = material.base_color;
    if (material.flags & MTOON_FLAGS_BASE_COLOR_TEXTURE) != 0u {
        base_color *= textureSampleBias(base_color_texture, base_color_sampler, in.uv, view.mip_bias);
    }

    // Alpha discard.
    if (material.flags & MTOON_FLAGS_ALPHA_MODE_OPAQUE) != 0u {
        base_color.a = 1.0;
    } else if (material.flags & MTOON_FLAGS_ALPHA_MODE_MASK) != 0u {
        if base_color.a >= material.alpha_cutoff {
            base_color.a = 1.0;
            pbr_input.material.alpha_cutoff = material.alpha_cutoff;
        } else {
            discard;
        }
    }

    // Normals.
    // Adapted from Bevy pbr_functions.
#ifndef LOAD_PREPASS_NORMALS
    // NOTE: The mikktspace method of normal mapping explicitly requires that the world normal NOT
    // be re-normalized in the fragment shader. This is primarily to match the way mikktspace
    // bakes vertex tangents and normal maps so that this is the exact inverse. Blender, Unity,
    // Unreal Engine, Godot, and more all use the mikktspace method. Do not change this code
    // unless you really know what you are doing.
    // http://www.mikktspace.com/
    var N: vec3<f32> = pbr_input.world_normal;

    if (material.flags & MTOON_FLAGS_NORMAL_MAP_TEXTURE) != 0u {

#ifdef VERTEX_TANGENTS
      // NOTE: The mikktspace method of normal mapping explicitly requires that these NOT be
      // normalized nor any Gram-Schmidt applied to ensure the vertex normal is orthogonal to the
      // vertex tangent! Do not change this code unless you really know what you are doing.
      // http://www.mikktspace.com/
      var T: vec3<f32> = in.world_tangent.xyz;
      var B: vec3<f32> = in.world_tangent.w * cross(N, T);
#endif

#ifdef VERTEX_TANGENTS
#ifdef VERTEX_UVS
      // Nt is the tangent-space normal.
      var Nt = textureSampleBias(normal_map_texture, normal_map_sampler, in.uv, view.mip_bias).rgb;
      Nt = Nt * 2.0 - 1.0;

      if double_sided && !is_front {
          Nt = -Nt;
      }

      // NOTE: The mikktspace method of normal mapping applies maps the tangent-space normal from
      // the normal map texture in this way to be an EXACT inverse of how the normal map baker
      // calculates the normal maps so there is no error introduced. Do not change this code
      // unless you really know what you are doing.
      // http://www.mikktspace.com/
      N = Nt.x * T + Nt.y * B + Nt.z * N;
#endif
#endif

      pbr_input.N = normalize(N);
    }
#endif

    // Emissive.
    var emissive = material.emissive_factor;
    if (material.flags & MTOON_FLAGS_EMISSIVE_TEXTURE) != 0u {
        emissive = vec4<f32>(emissive.rgb * textureSampleBias(emissive_texture, emissive_sampler, in.uv, view.mip_bias).rgb, 1.0);
    }
    pbr_input.material.emissive = emissive;

    // Shading.
    var shading = dot(pbr_input.N, material.light_dir);
    shading = shading + material.shading_shift_factor;
    if (material.flags & MTOON_FLAGS_SHADING_SHIFT_TEXTURE) != 0u {
        // Is grabbing the alpha correct here?
        shading = shading + textureSampleBias(shade_shift_texture, shade_shift_sampler, in.uv, view.mip_bias).a;
    }
    shading = 1.0 - linear_step(material.shading_toony_factor - 1.0, 1.0 - material.shading_toony_factor, shading);
    var shade_color = material.shade_color;
    if (material.flags & MTOON_FLAGS_SHADE_COLOR_TEXTURE) != 0u {
        shade_color *= textureSampleBias(shade_color_texture, shade_color_sampler, in.uv, view.mip_bias).rgb;
    }
    var mtoon_rgb = mix(base_color.rgb, shade_color, shading);
    mtoon_rgb *= material.light_color;

    // Global illumination.
    pbr_input.material.base_color = vec4<f32>(mtoon_rgb, base_color.a);
    let pbr_lighting_color = apply_pbr_lighting(pbr_input);
    let n_dot_v = max(dot(pbr_input.N, pbr_input.V), 0.0001);

    let diffuse_color = mtoon_rgb.rgb;
    let F0 = base_color.rgb;
    let perceptual_roughness = pbr_input.material.perceptual_roughness;
    let diffuse_occlusion = pbr_input.diffuse_occlusion;
    var uniform_gi = ambient_light(pbr_input.world_position, pbr_input.N, pbr_input.V, n_dot_v, diffuse_color, F0, perceptual_roughness, diffuse_occlusion);

    uniform_gi *= base_color.rgb;
    uniform_gi *= view.exposure;

    let gi = mix(pbr_lighting_color.rgb, uniform_gi, material.gi_equalization_factor);
    mtoon_rgb += gi;

    // Rim lighting.
    var rim = vec3(0.0);
    if (material.flags & MTOON_FLAGS_MATCAP_TEXTURE) != 0u {
        let world_view_x = normalize(vec3<f32>(pbr_input.V.z, 0.0, -pbr_input.V.x));
        let world_view_y = cross(pbr_input.V, world_view_x);
        let matcap_uv = vec2<f32>(dot(world_view_x, pbr_input.N), dot(world_view_y, pbr_input.N)) * 0.495 + 0.5;
        let matcap_color = textureSampleBias(matcap_texture, matcap_sampler, matcap_uv, view.mip_bias);
        rim = material.matcap_factor * matcap_color.rgb;
    }
    var parametric_rim = saturate(1.0 - dot(pbr_input.N, pbr_input.V) + material.parametric_rim_lift_factor);
    parametric_rim = pow(parametric_rim, max(material.parametric_rim_fresnel_power, EPSILON));
    rim += parametric_rim * material.parametric_rim_color;
    if (material.flags & MTOON_FLAGS_RIM_MULTIPLY_TEXTURE) != 0u {
        let rim_multiply = textureSampleBias(rim_multiply_texture, rim_multiply_sampler, in.uv, view.mip_bias);
        rim *= rim_multiply.rgb;
    }
    rim *= mix(vec3(1.0), pbr_lighting_color.rgb, material.rim_lighting_mix_factor);
    mtoon_rgb += rim;

    return vec4<f32>(mtoon_rgb, base_color.a);
}

fn linear_step(a: f32, b: f32, t: f32) -> f32 {
    return saturate((t - a) / (b - a));
}
