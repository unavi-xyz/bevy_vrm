use bevy::{
    asset::{LoadContext, LoadedAsset},
    prelude::*,
    utils::HashMap,
};
use bevy_shader_mtoon::MtoonMaterial;

use crate::{extensions::vrm0::MaterialProperty, MtoonReplaceMat};

const MTOON_KEY: &str = "VRM/MToon";

pub fn load_gltf(
    gltf: &goth_gltf::Gltf<super::Extensions>,
    load_context: &mut LoadContext,
) -> Result<(), Box<dyn std::error::Error>> {
    let vrm0 = gltf.extensions.vrm0.as_ref();

    if vrm0.is_none() {
        return Err("VRM0 not found".into());
    }

    let vrm0 = vrm0.unwrap();

    let mut mtoon_materials = HashMap::<usize, Handle<MtoonMaterial>>::new();

    if let Some(material_properties) = &vrm0.material_properties {
        material_properties
            .iter()
            .enumerate()
            .for_each(|(i, material_property)| {
                if let Some(shader) = &material_property.shader {
                    match shader.as_str() {
                        MTOON_KEY => {
                            let handle = load_mtoon(material_property, i, load_context);
                            mtoon_materials.insert(i, handle);
                        }
                        _ => {
                            warn!("Unknown shader: {}", shader);
                        }
                    }
                }
            });
    }

    // Mark materials as needing replacement
    gltf.meshes.iter().enumerate().for_each(|(i, mesh)| {
        let mesh_label = mesh_label(i);

        mesh.primitives
            .iter()
            .enumerate()
            .for_each(|(j, primitive)| {
                if let Some(mat_index) = primitive.material {
                    if let Some(mtoon_handle) = mtoon_materials.get(&mat_index) {
                        let replace_label = format!("Replace{mat_index}");

                        let mesh_handle = load_context.get_label_handle(&mesh_label);

                        load_context.add_loaded_labeled_asset(
                            replace_label,
                            LoadedAsset::new_with_dependencies(
                                MtoonReplaceMat {
                                    mesh: mesh_handle,
                                    primitive: j,
                                    mtoon: mtoon_handle.clone(),
                                },
                                None,
                            ),
                        );
                    }
                }
            });
    });

    Ok(())
}

/// Loads a VRM/MToon material and returns it
fn load_mtoon(
    property: &MaterialProperty,
    index: usize,
    load_context: &mut LoadContext,
) -> Handle<MtoonMaterial> {
    // info!("Loading MToon material: {:#?}", property);

    let mtoon_label = mtoon_label(index);

    let mut mtoon_material = MtoonMaterial::default();

    if let Some(float) = &property.float {
        if let Some(shade_shift) = float.shade_shift {
            mtoon_material.shading_shift_factor = shade_shift;
        }

        if let Some(shade_toony) = float.shade_toony {
            mtoon_material.shading_toony_factor = shade_toony;
        }
    }

    if let Some(vector) = &property.vector {
        if let Some(color) = vector.color {
            mtoon_material.base_color = color.into();
        }

        if let Some(color) = vector.shade_color {
            mtoon_material.shade_color = color.into();
        }
    }

    if let Some(texture) = &property.texture {
        if let Some(main_tex) = texture.main_tex {
            let label = texture_label(main_tex);

            if load_context.has_labeled_asset(&label) {
                let handle = load_context.get_label_handle(&label);
                mtoon_material.base_color_texture = Some(handle);
            }
        }

        if let Some(shade_texture) = texture.shade_texture {
            let label = texture_label(shade_texture);

            if load_context.has_labeled_asset(&label) {
                let handle = load_context.get_label_handle(&label);
                mtoon_material.shade_color_texture = Some(handle);
            }
        }
    }

    load_context.add_loaded_labeled_asset(
        mtoon_label,
        LoadedAsset::new_with_dependencies(mtoon_material, None),
    )
}

fn mesh_label(index: usize) -> String {
    format!("Mesh{index}")
}

fn mtoon_label(index: usize) -> String {
    format!("Material{MTOON_KEY}{index}")
}

fn texture_label(index: u32) -> String {
    format!("Texture{index}")
}
