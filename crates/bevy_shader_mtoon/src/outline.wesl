import bevy_pbr::render::{
    mesh_bindings::mesh,
    mesh_functions,
    skinning,
    morph::{morph_position, morph_normal},
    forward_io::{Vertex, UncompressedVertex, decompress_vertex},
    view_transformations::{position_world_to_clip, direction_world_to_view},
    mesh_view_bindings::view,
};

struct OutlineMaterialUniform {
    color: vec4<f32>,
    width: f32,
    mode: u32,
};

@group(constants::MATERIAL_BIND_GROUP) @binding(0)
var<uniform> outline: OutlineMaterialUniform;

const OUTLINE_MODE_WORLD: u32 = 1u;
const OUTLINE_MODE_SCREEN: u32 = 2u;
const EPSILON: f32 = 0.0001;

@if(MORPH_TARGETS)
fn morph_vertex(vertex_in: UncompressedVertex, instance_index: u32) -> UncompressedVertex {
    var vertex = vertex_in;
    let first_vertex = mesh[instance_index].first_vertex_index;
    let vertex_index = vertex.index - first_vertex;
    let morph_descriptor_index = mesh[instance_index].morph_descriptor_index;

    let weight_count = bevy_pbr::render::morph::layer_count(morph_descriptor_index);
    for (var i: u32 = 0u; i < weight_count; i++) {
        let weight = bevy_pbr::render::morph::weight_at(i, morph_descriptor_index);
        if weight == 0.0 {
            continue;
        }
        vertex.position += weight * morph_position(vertex_index, i, morph_descriptor_index);
        @if(VERTEX_NORMALS)
        vertex.normal += weight * morph_normal(vertex_index, i, morph_descriptor_index);
    }
    return vertex;
}

@vertex
fn vertex(vertex_no_morph: Vertex) -> @builtin(position) vec4<f32> {
    let uncompressed_vertex_no_morph = decompress_vertex(vertex_no_morph, vertex_no_morph.instance_index);
@if(MORPH_TARGETS)
    var vertex = morph_vertex(uncompressed_vertex_no_morph, vertex_no_morph.instance_index);
@else
    var vertex = uncompressed_vertex_no_morph;

@if(SKINNED)
    let world_from_local = skinning::skin_model(
        vertex.joint_indices,
        vertex.joint_weights,
        vertex_no_morph.instance_index
    );
@else
    let world_from_local = mesh_functions::get_world_from_local(vertex_no_morph.instance_index);
@if(SKINNED)
    let world_normal = skinning::skin_normals(world_from_local, vertex.normal);
@else
    let world_normal = mesh_functions::mesh_normal_local_to_world(vertex.normal, vertex_no_morph.instance_index);

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
