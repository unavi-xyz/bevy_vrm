struct MtoonMaterial {
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

@group(1) @binding(0)
var<uniform> material: MtoonMaterial;
@group(1) @binding(1)
var base_color_texture: texture_2d<f32>;
@group(1) @binding(2)
var base_color_sampler: sampler;
@group(1) @binding(3)
var shade_color_texture: texture_2d<f32>;
@group(1) @binding(4)
var shade_color_sampler: sampler;
//var shading_shift_texture: texture_2d<f32>;
//var shading_shift_sampler: sampler;
//var normal_texture: texture_2d<f32>;
//var normal_sampler: sampler;
//var emissive_texture: texture_2d<f32>;
//var emissive_sampler: sampler;
@group(1) @binding(5)
var matcap_texture: texture_2d<f32>;
@group(1) @binding(6)
var matcap_sampler: sampler;
@group(1) @binding(7)
var rim_multiply_texture: texture_2d<f32>;
@group(1) @binding(8)
var rim_multiply_sampler: sampler;

#import bevy_pbr::forward_io::VertexOutput

const RIM_EPSILON = 0.00001;
const VEC4_ONE = vec4<f32>(1.0, 1.0, 1.0, 1.0);

@fragment
fn fragment (in: VertexOutput) -> @location(0) vec4<f32> {
    // Base lighting
    // TODO: Doesn't use shading_shift_texture
    let base_color = material.base_color * textureSample(base_color_texture, base_color_sampler, in.uv);
    let shade_color = material.shade_color * textureSample(shade_color_texture, shade_color_sampler, in.uv);

    let normal = normalize(in.world_normal);
    let n_dot_l = dot(normal, material.light_dir);

    let base_shading = n_dot_l + material.shading_shift_factor;
    let shading = 1.0 - linear_step(material.shading_toony_factor - 1.0, 1.0 - material.shading_toony_factor, base_shading);

    var color = mix(base_color, shade_color, shading) * material.light_color;

    // Global illumination
    // TODO: This is not at all correct
    let uniform_gi = material.ambient_color.a / 2.0;
    let passthrough_gi = n_dot_l * material.ambient_color.a;
    let gi = shading * mix(passthrough_gi, uniform_gi, material.gl_equalization_factor);
    color = color + gi * material.ambient_color;
  
    // TODO: Emission

    // Rim lighting
    let world_view_x = normalize(vec3<f32>(material.view_dir.z, 0.0, -material.view_dir.x));
    let world_view_y = cross(material.view_dir, world_view_x);

    let matcap_uv = vec2<f32>(dot(world_view_x, normal), dot(world_view_y, normal));

    var rim = material.matcap_factor * textureSample(matcap_texture, matcap_sampler, matcap_uv);

    var parametric_rim = saturate(1.0 - dot(normal, material.view_dir) + material.parametric_rim_lift_factor);
    parametric_rim = pow(parametric_rim, max(material.parametric_rim_fresnel_power, RIM_EPSILON));

    rim = rim + parametric_rim * material.parametric_rim_color;
    rim = rim * textureSample(rim_multiply_texture, rim_multiply_sampler, in.uv);
    rim = rim * mix(VEC4_ONE, material.light_color, material.rim_lighting_mix_factor);

    color = color + rim;

    return color;
}

fn linear_step(a: f32, b: f32, t: f32) -> f32 {
    return saturate((t - a) / (b - a));
}
