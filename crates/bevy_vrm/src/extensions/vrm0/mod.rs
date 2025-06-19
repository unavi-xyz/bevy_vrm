use bevy::{asset::LoadedAsset, prelude::*};
use bevy_gltf_kun::import::gltf::document::ImportContext;
use bevy_shader_mtoon::{MtoonMaterial, OutlineMode, OutlineSync};
use gltf_kun::graph::{
    ByteNode,
    gltf::{Material, Primitive},
};
use gltf_kun_vrm::vrm0::{Vrm, material_property::MaterialProperty};
use serde_vrm::vrm0::Shader;

pub fn import_material(context: &mut ImportContext, material: Material, ext: Vrm) {
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
                    let mtoon = load_mtoon_shader(context, *material_property);

                    context.load_context.add_loaded_labeled_asset(
                        label,
                        LoadedAsset::new_with_dependencies(mtoon, None),
                    );
                }
            }
            Some(Shader::Gltf) => {}
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
    material_property: MaterialProperty,
) -> MtoonMaterial {
    let mut mtoon = MtoonMaterial::default();

    let weight = material_property.read(context.graph);

    if let Some(value) = weight.float.double_sided {
        mtoon.double_sided = value == 0.0;
    }

    if let Some(value) = weight.float.cutoff {
        mtoon.alpha_mode = AlphaMode::Mask(value);
    }

    if let Some(value) = weight.vector.color {
        mtoon.base_color = LinearRgba::from_f32_array(value).into();
    }

    if let Some(texture) = material_property.main_texture(context.graph) {
        let index = context.doc.texture_index(context.graph, texture).unwrap();
        let label = texture_label(index);
        let handle = context.load_context.get_label_handle(&label);
        mtoon.base_color_texture = Some(handle);
    }

    if let Some(value) = weight.float.normal_scale {
        mtoon.normal_map_scale = value;
    }

    if let Some(texture) = material_property.bump_map(context.graph) {
        let index = context.doc.texture_index(context.graph, texture).unwrap();
        let label = texture_label(index);
        let handle = context.load_context.get_label_handle(&label);
        mtoon.normal_map_texture = Some(handle);
    }

    if let Some(value) = weight.vector.emissive_factor {
        mtoon.emissive_factor = LinearRgba::from_f32_array(value).into();
    }

    if let Some(texture) = material_property.emission_map(context.graph) {
        let index = context.doc.texture_index(context.graph, texture).unwrap();
        let label = texture_label(index);
        let handle = context.load_context.get_label_handle(&label);
        mtoon.emissive_texture = Some(handle);
    }

    if let Some(value) = weight.float.outline_factor {
        mtoon.outline_width = value;
    }

    if let Some(value) = weight.vector.outline_color {
        mtoon.outline_color = LinearRgba::from_f32_array(value).into();
    }

    if let Some(value) = weight.keyword_map.outline_width_world {
        if value {
            mtoon.outline_mode = OutlineMode::World;
        } else {
            mtoon.outline_mode = OutlineMode::Screen;
        }
    }

    if let Some(value) = weight.float.gi_intensity_factor {
        mtoon.gi_equalization_factor = 1.0 - value;
    }

    if let Some(value) = weight.float.shade_shift {
        mtoon.shading_shift_factor = -value;
    }

    if let Some(value) = weight.float.shade_toony {
        mtoon.shading_toony_factor = value;
    }

    if let Some(value) = weight.vector.shade_color {
        mtoon.shade_factor = LinearRgba::from_f32_array(value).into();
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
        mtoon.shade_multiply_texture = Some(handle);
    }

    mtoon
}

fn mtoon_label(index: usize) -> String {
    format!("MaterialMtoon{}", index)
}

fn texture_label(index: usize) -> String {
    format!("Texture{}", index)
}
