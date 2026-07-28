#define_import_path bevy_shader_mtoon::outline

#import bevy_pbr::{
    mesh_bindings::mesh,
    mesh_functions,
    skinning,
    morph::{morph_position, morph_normal},
    forward_io::Vertex,
    view_transformations::{position_world_to_clip, direction_world_to_view},
    mesh_view_bindings::view,
}

struct OutlineMaterialUniform {
    color: vec4<f32>,
    width: f32,
    mode: u32,
};

@group(#{MATERIAL_BIND_GROUP}) @binding(0)
var<uniform> outline: OutlineMaterialUniform;

const OUTLINE_MODE_WORLD: u32 = 1u;
const OUTLINE_MODE_SCREEN: u32 = 2u;
const EPSILON: f32 = 0.0001;

#ifdef MORPH_TARGETS
fn morph_vertex(vertex_in: Vertex, instance_index: u32) -> Vertex {
    var vertex = vertex_in;
    let first_vertex = mesh[instance_index].first_vertex_index;
    let vertex_index = vertex.index - first_vertex;

    let weight_count = bevy_pbr::morph::layer_count(instance_index);
    for (var i: u32 = 0u; i < weight_count; i++) {
        let weight = bevy_pbr::morph::weight_at(i, instance_index);
        if weight == 0.0 {
            continue;
        }
        vertex.position += weight * morph_position(vertex_index, i, instance_index);
#ifdef VERTEX_NORMALS
        vertex.normal += weight * morph_normal(vertex_index, i, instance_index);
#endif
    }
    return vertex;
}
#endif

@vertex
fn vertex(vertex_no_morph: Vertex) -> @builtin(position) vec4<f32> {
#ifdef MORPH_TARGETS
    var vertex = morph_vertex(vertex_no_morph, vertex_no_morph.instance_index);
#else
    var vertex = vertex_no_morph;
#endif

#ifdef SKINNED
    let world_from_local = skinning::skin_model(
        vertex.joint_indices,
        vertex.joint_weights,
        vertex_no_morph.instance_index
    );
    let world_normal = skinning::skin_normals(world_from_local, vertex.normal);
#else
    let world_from_local = mesh_functions::get_world_from_local(vertex_no_morph.instance_index);
    let world_normal = mesh_functions::mesh_normal_local_to_world(vertex.normal, vertex_no_morph.instance_index);
#endif

    let world_position = mesh_functions::mesh_position_local_to_world(
        world_from_local,
        vec4<f32>(vertex.position, 1.0)
    ).xyz;
    let n = normalize(world_normal);

    if outline.mode == OUTLINE_MODE_WORLD {
        return position_world_to_clip(world_position + n * outline.width);
    }

    let clip = position_world_to_clip(world_position);
    var offset = normalize(direction_world_to_view(n).xy + vec2<f32>(EPSILON));
    offset.x *= view.viewport.w / view.viewport.z;
    return vec4<f32>(clip.xy + offset * outline.width * clip.w, clip.zw);
}

@fragment
fn fragment() -> @location(0) vec4<f32> {
    return outline.color;
}
