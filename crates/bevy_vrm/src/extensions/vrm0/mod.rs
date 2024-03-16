use bevy::{asset::LoadedAsset, prelude::*};
use bevy_gltf_kun::import::gltf::{document::ImportContext, texture::texture_label};
use bevy_shader_mtoon::{MtoonMaterial, MtoonShader};
use gltf_kun::graph::{gltf::Primitive, ByteNode, GraphNodeWeight};
use gltf_kun_vrm::vrm0::{material_property::MaterialProperty, Vrm};
use serde_vrm::vrm0::Shader;

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

                let handle = if context.load_context.has_labeled_asset(label.clone()) {
                    context
                        .load_context
                        .get_label_handle::<MtoonMaterial>(&label)
                } else {
                    let shader = load_mtoon_shader(context, *material_property);

                    let mtoon = MtoonMaterial {
                        base: StandardMaterial::default(),
                        extension: shader,
                    };

                    context.load_context.add_loaded_labeled_asset(
                        label,
                        LoadedAsset::new_with_dependencies(mtoon, None),
                    )
                };

                entity.remove::<Handle<StandardMaterial>>().insert(handle);
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
) -> MtoonShader {
    let mut shader = MtoonShader::default();

    let weight = material_property.read(context.graph);

    // TODO: This doesn't actually change the material, since this function is called after the material has been processed.
    if let Some(mut material) = material_property.material(context.graph) {
        let m_weight = material.get_mut(context.graph);

        if let Some(value) = weight.vector.color {
            m_weight.base_color_factor = value;
        }

        if let Some(texture) = material_property.main_texture(context.graph) {
            material.set_base_color_texture(context.graph, Some(texture));
        }
    }

    if let Some(value) = weight.float.shade_shift {
        shader.shading_shift_factor = value;
    }

    if let Some(value) = weight.float.shade_toony {
        shader.shading_toony_factor = value;
    }

    if let Some(value) = weight.vector.shade_color {
        shader.shade_color = Color::rgba_linear_from_array(value);
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
        shader.shade_color_texture = Some(handle);
    }

    shader
}

fn mtoon_label(index: usize) -> String {
    format!("MaterialMtoon{}", index)
}
