use bevy::{asset::LoadedAsset, prelude::*};
use bevy_gltf_kun::import::gltf::document::ImportContext;
use bevy_shader_mtoon::{MtoonMaterial, MtoonShader, OutlineSync};
use gltf_kun::graph::{
    gltf::{Material, Primitive},
    ByteNode,
};
use gltf_kun_vrm::vrm0::{material_property::MaterialProperty, Vrm};
use serde_vrm::vrm0::Shader;

pub fn import_material(
    context: &mut ImportContext,
    standard_material: &mut StandardMaterial,
    material: Material,
    ext: Vrm,
) {
    for (i, material_property) in ext.material_properties(context.graph).iter().enumerate() {
        let m = match material_property.material(context.graph) {
            Some(material) => material,
            None => {
                warn!("Material not found for property {}", i);
                continue;
            }
        };

        if m.0 != material.0 {
            continue;
        }

        let weight = material_property.read(context.graph);

        match weight.shader {
            Some(Shader::MToon) => {
                let label = mtoon_label(i);

                if !context.load_context.has_labeled_asset(label.clone()) {
                    let mut base = standard_material.clone();

                    let shader = load_mtoon_shader(context, &mut base, *material_property);

                    let mtoon = MtoonMaterial {
                        base,
                        extension: shader,
                    };

                    context.load_context.add_loaded_labeled_asset(
                        label,
                        LoadedAsset::new_with_dependencies(mtoon, None),
                    );
                }
            }
            Some(other) => {
                warn!("Unsupported shader: {:?}", other);
            }
            None => {}
        }
    }
}

pub fn import_primitive_material(
    context: &mut ImportContext,
    entity: &mut EntityWorldMut,
    ext: Vrm,
    primitive: Primitive,
) {
    let primitive_material = match primitive.material(context.graph) {
        Some(material) => material,
        None => return,
    };

    for (i, material_property) in ext.material_properties(context.graph).iter().enumerate() {
        let material = match material_property.material(context.graph) {
            Some(material) => material,
            None => {
                warn!("Material not found for property {}", i);
                continue;
            }
        };

        if material.0 != primitive_material.0 {
            continue;
        }

        let weight = material_property.read(context.graph);

        match weight.shader {
            Some(Shader::MToon) => {
                let label = mtoon_label(i);

                if !context.load_context.has_labeled_asset(label.clone()) {
                    warn!("MToon material not found for property {}", i);
                    continue;
                }

                let handle = context
                    .load_context
                    .get_label_handle::<MtoonMaterial>(&label);

                entity
                    .remove::<Handle<StandardMaterial>>()
                    .insert((handle, OutlineSync));
            }
            Some(other) => {
                warn!("Unsupported shader: {:?}", other);
            }
            None => {}
        }
    }
}

fn load_mtoon_shader(
    context: &mut ImportContext,
    base: &mut StandardMaterial,
    material_property: MaterialProperty,
) -> MtoonShader {
    let mut shader = MtoonShader::default();

    let weight = material_property.read(context.graph);

    if let Some(value) = weight.vector.color {
        base.base_color = Color::rgba_linear_from_array(value);
    }

    if let Some(texture) = material_property.main_texture(context.graph) {
        let index = context.doc.texture_index(context.graph, texture).unwrap();
        let label = texture_label(index);
        let handle = context.load_context.get_label_handle(&label);
        base.base_color_texture = Some(handle);
    }

    if let Some(texture) = material_property.bump_map(context.graph) {
        let index = context.doc.texture_index(context.graph, texture).unwrap();
        let label = texture_label(index);
        let handle = context.load_context.get_label_handle(&label);
        base.normal_map_texture = Some(handle);

        // TODO: Bump map scale
    }

    if let Some(texture) = material_property.emission_map(context.graph) {
        let index = context.doc.texture_index(context.graph, texture).unwrap();
        let label = texture_label(index);
        let handle = context.load_context.get_label_handle(&label);
        base.emissive_texture = Some(handle);
    }

    if let Some(value) = weight.float.indirect_light_intensity {
        shader.gi_equalization_factor = 1.0 - value;
    }

    if let Some(value) = weight.float.shade_shift {
        shader.shading_shift_factor = -value;
    }

    if let Some(value) = weight.float.shade_toony {
        shader.shading_toony_factor = value;
    }

    if let Some(value) = weight.vector.shade_color {
        shader.shade_factor = Color::rgba_linear_from_array(value);
    }

    if let Some(texture) = material_property.shade_texture(context.graph) {
        let index = context
            .doc
            .textures(context.graph)
            .iter()
            .position(|t| t.0 == texture.0)
            .unwrap();
        let label = texture_label(index);
        let handle = context.load_context.get_label_handle(&label);
        shader.shade_multiply_texture = Some(handle);
    }

    shader
}

fn mtoon_label(index: usize) -> String {
    format!("MaterialMtoon{}", index)
}

fn texture_label(index: usize) -> String {
    format!("Texture{}", index)
}
