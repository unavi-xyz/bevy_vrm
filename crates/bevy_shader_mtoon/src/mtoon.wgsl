#define_import_path bevy_shader_mtoon::mtoon

#import bevy_pbr::{
    pbr_fragment::pbr_input_from_vertex_output,
    mesh_view_bindings::view,
    ambient::ambient_light,
}

#ifdef PREPASS_PIPELINE
#import bevy_pbr::prepass_io::VertexOutput
#else
#import bevy_pbr::forward_io::VertexOutput
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

@group(#{MATERIAL_BIND_GROUP}) @binding(0)
var<uniform> material: MtoonMaterialUniform;

@group(#{MATERIAL_BIND_GROUP}) @binding(1) var base_color_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var base_color_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var emissive_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(4) var emissive_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(5) var matcap_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(6) var matcap_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(7) var normal_map_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(8) var normal_map_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(9) var rim_multiply_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(10) var rim_multiply_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(11) var shade_color_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(12) var shade_color_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(13) var shade_shift_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(14) var shade_shift_sampler: sampler;

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

fn mtoon_shade(in: VertexOutput, is_front: bool) -> vec4<f32> {
    let double_sided = (material.flags & MTOON_FLAGS_DOUBLE_SIDED) != 0u;
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

    // Normal mapping. The mikktspace method requires the world normal is NOT re-normalized before
    // applying the tangent-space normal. http://www.mikktspace.com/
#ifndef LOAD_PREPASS_NORMALS
    var N = pbr_input.world_normal;
    if (material.flags & MTOON_FLAGS_NORMAL_MAP_TEXTURE) != 0u {
#ifdef VERTEX_TANGENTS
#ifdef VERTEX_UVS
        let T = in.world_tangent.xyz;
        let B = in.world_tangent.w * cross(N, T);
        var Nt = textureSampleBias(normal_map_texture, normal_map_sampler, in.uv, view.mip_bias).rgb * 2.0 - 1.0;
        Nt = vec3<f32>(Nt.xy * material.normal_map_scale, Nt.z);
        if double_sided && !is_front {
            Nt = -Nt;
        }
        N = Nt.x * T + Nt.y * B + Nt.z * N;
#endif
#endif
        pbr_input.N = normalize(N);
    }
#endif

    let n = pbr_input.N;
    let v = pbr_input.V;
    let n_dot_v = max(dot(n, v), 0.0001);
    let occlusion = pbr_input.diffuse_occlusion;
    let roughness = pbr_input.material.perceptual_roughness;

    // Toon shading: interpolate base and shade colors by how lit the surface is.
    var shade_color = material.shade_color;
    if (material.flags & MTOON_FLAGS_SHADE_COLOR_TEXTURE) != 0u {
        shade_color *= textureSampleBias(shade_color_texture, shade_color_sampler, in.uv, view.mip_bias).rgb;
    }

    var shading = dot(n, material.light_dir) + material.shading_shift_factor;
    if (material.flags & MTOON_FLAGS_SHADING_SHIFT_TEXTURE) != 0u {
        shading += textureSampleBias(shade_shift_texture, shade_shift_sampler, in.uv, view.mip_bias).r;
    }
    shading = linear_step(-1.0 + material.shading_toony_factor, 1.0 - material.shading_toony_factor, shading);

    var color = mix(shade_color, base_color.rgb, shading);
    color *= material.light_color;

    // Global illumination, equalized so shaded regions keep constant ambient regardless of facing.
    // MToon is diffuse-only, so the specular ambient term is suppressed with a zero F0.
    let f0 = vec3<f32>(0.0);
    let raw_gi = ambient_light(pbr_input.world_position, n, v, n_dot_v, base_color.rgb, f0, roughness, occlusion);
    let gi_up = ambient_light(pbr_input.world_position, vec3<f32>(0.0, 1.0, 0.0), v, n_dot_v, base_color.rgb, f0, roughness, occlusion);
    let gi_down = ambient_light(pbr_input.world_position, vec3<f32>(0.0, -1.0, 0.0), v, n_dot_v, base_color.rgb, f0, roughness, occlusion);
    let uniformed_gi = (gi_up + gi_down) * 0.5;
    color += mix(raw_gi, uniformed_gi, material.gi_equalization_factor) * view.exposure;

    let lighting = color;

    // Emissive.
    var emissive = material.emissive_factor.rgb;
    if (material.flags & MTOON_FLAGS_EMISSIVE_TEXTURE) != 0u {
        emissive *= textureSampleBias(emissive_texture, emissive_sampler, in.uv, view.mip_bias).rgb;
    }
    color += emissive;

    // Rim lighting.
    var rim = vec3<f32>(0.0);
    if (material.flags & MTOON_FLAGS_MATCAP_TEXTURE) != 0u {
        let world_view_x = normalize(vec3<f32>(v.z, 0.0, -v.x));
        let world_view_y = cross(v, world_view_x);
        let matcap_uv = vec2<f32>(dot(world_view_x, n), dot(world_view_y, n)) * 0.495 + 0.5;
        rim = material.matcap_factor * textureSampleBias(matcap_texture, matcap_sampler, matcap_uv, view.mip_bias).rgb;
    }
    var parametric_rim = saturate(1.0 - dot(n, v) + material.parametric_rim_lift_factor);
    parametric_rim = pow(parametric_rim, max(material.parametric_rim_fresnel_power, EPSILON));
    rim += parametric_rim * material.parametric_rim_color;
    if (material.flags & MTOON_FLAGS_RIM_MULTIPLY_TEXTURE) != 0u {
        rim *= textureSampleBias(rim_multiply_texture, rim_multiply_sampler, in.uv, view.mip_bias).rgb;
    }
    rim *= mix(vec3<f32>(1.0), lighting, material.rim_lighting_mix_factor);
    color += rim;

    return vec4<f32>(color, base_color.a);
}

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> @location(0) vec4<f32> {
    return mtoon_shade(in, is_front);
}

fn linear_step(a: f32, b: f32, t: f32) -> f32 {
    return saturate((t - a) / (b - a));
}
