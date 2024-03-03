use bevy::{
    asset::{LoadContext, LoadedAsset},
    prelude::*,
};
use bevy_shader_mtoon::MtoonMaterial;

use crate::{extensions::vrm0::MaterialProperty, MtoonMarker, Vrm};

const MTOON_KEY: &str = "VRM/MToon";

pub(crate) fn load_gltf(
    vrm: &mut Vrm,
    goth_gltf: &goth_gltf::Gltf<super::Extensions>,
    load_context: &mut LoadContext,
) -> Result<(), Box<dyn std::error::Error>> {
    let vrm0 = match &goth_gltf.extensions.vrm0 {
        Some(vrm0) => vrm0,
        None => return Err("VRM0 not found".into()),
    };

    // Load MToon materials
    if let Some(material_properties) = &vrm0.material_properties {
        material_properties
            .iter()
            .enumerate()
            .for_each(|(i, material_property)| {
                if let Some(shader) = &material_property.shader {
                    match shader.as_str() {
                        MTOON_KEY => {
                            let handle = load_mtoon(material_property, i, load_context);
                            vrm.mtoon_materials.insert(i, handle);
                        }
                        _ => {
                            warn!("Unknown shader: {}", shader);
                        }
                    }
                }
            });
    }

    // Create MtoonMarkers
    goth_gltf
        .meshes
        .iter()
        .enumerate()
        .for_each(|(mesh_index, mesh)| {
            let mesh_label = mesh_label(mesh_index);

            mesh.primitives
                .iter()
                .enumerate()
                .for_each(|(primitive_index, primitive)| {
                    let material_index = match primitive.material {
                        Some(material_index) => material_index,
                        None => return,
                    };

                    let mtoon_material = match vrm.mtoon_materials.get(&material_index) {
                        Some(mtoon_material) => mtoon_material,
                        None => return,
                    };

                    if !load_context.has_labeled_asset(&mesh_label) {
                        warn!("Mesh not loaded: {}", mesh_label);
                        return;
                    }

                    vrm.mtoon_markers.push(MtoonMarker {
                        mesh: load_context.get_label_handle(&mesh_label),
                        primitive: primitive_index,
                        mtoon: mtoon_material.clone(),
                    });
                });
        });

    vrm.extensions = goth_gltf.extensions.clone();

    Ok(())
}

/// Loads a VRM/MToon material and returns a handle
fn load_mtoon(
    property: &MaterialProperty,
    index: usize,
    load_context: &mut LoadContext,
) -> Handle<MtoonMaterial> {
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
            mtoon_material.base_color = Color::rgba_from_array(color);
        }

        if let Some(color) = vector.shade_color {
            mtoon_material.shade_color = Color::rgba_from_array(color);
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
