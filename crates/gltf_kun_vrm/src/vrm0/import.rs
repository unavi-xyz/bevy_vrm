use gltf_kun::{
    extensions::{Extension, ExtensionImport},
    graph::{gltf::GltfDocument, ByteNode, Extensions, Graph},
    io::format::gltf::GltfFormat,
};
use thiserror::Error;

use super::{
    bind::{Bind, BindWeight},
    blend_shape_group::{BlendShapeGroup, BlendShapeGroupWeight},
    bone::{Bone, BoneWeight},
    bone_group::{BoneGroup, BoneGroupWeight},
    collider_group::{ColliderGroup, ColliderGroupWeight},
    material_property::{MaterialProperty, MaterialPropertyWeight},
    weight::{FirstPerson, Humanoid, Meta, VrmWeight},
    Vrm, EXTENSION_NAME,
};

#[derive(Debug, Error)]
pub enum VrmImportError {
    #[error("Material not found: {0}")]
    MaterialNotFound(usize),
    #[error("Node not found: {0}")]
    NodeNotFound(usize),
    #[error("Texture not found: {0}")]
    TextureNotFound(usize),
    #[error("Bone not found: {0}")]
    BoneNotFound(usize),
    #[error("Bone group not found: {0}")]
    BoneGroupNotFound(usize),
    #[error("Collider group not found: {0}")]
    ColliderGroupNotFound(usize),
}

impl ExtensionImport<GltfDocument, GltfFormat> for Vrm {
    fn import(
        graph: &mut Graph,
        format: &mut GltfFormat,
        doc: &GltfDocument,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let extensions = match &format.json.extensions {
            Some(extensions) => extensions,
            None => return Ok(()),
        };

        let ext = match extensions.others.get(EXTENSION_NAME) {
            Some(ext) => ext,
            None => return Ok(()),
        };

        let ext: serde_vrm::vrm0::Vrm = serde_json::from_value(ext.clone())?;

        let vrm = Vrm::new(graph);
        doc.add_extension(graph, vrm);

        for (i, material_property_json) in ext
            .material_properties
            .unwrap_or_default()
            .into_iter()
            .enumerate()
        {
            let material = doc
                .materials(graph)
                .get(i)
                .copied()
                .ok_or_else(|| Box::new(VrmImportError::MaterialNotFound(i)))?;

            let material_property = MaterialProperty::new(graph);
            vrm.add_material_property(graph, material_property);

            material_property.set_material(graph, Some(material));

            if let Some(texture_properties) = material_property_json.texture {
                if let Some(idx) = texture_properties.base_color {
                    doc.textures(graph)
                        .get(idx as usize)
                        .map(|texture| {
                            material_property.set_main_texture(graph, Some(*texture));
                        })
                        .ok_or_else(|| Box::new(VrmImportError::TextureNotFound(idx as usize)))?;
                }

                if let Some(idx) = texture_properties.shade {
                    doc.textures(graph)
                        .get(idx as usize)
                        .map(|texture| {
                            material_property.set_shade_texture(graph, Some(*texture));
                        })
                        .ok_or_else(|| Box::new(VrmImportError::TextureNotFound(idx as usize)))?;
                }

                if let Some(idx) = texture_properties.additive {
                    doc.textures(graph)
                        .get(idx as usize)
                        .map(|texture| {
                            material_property.set_sphere_add_texture(graph, Some(*texture));
                        })
                        .ok_or_else(|| Box::new(VrmImportError::TextureNotFound(idx as usize)))?;
                }

                if let Some(idx) = texture_properties.normal {
                    doc.textures(graph)
                        .get(idx as usize)
                        .map(|texture| {
                            material_property.set_bump_map(graph, Some(*texture));
                        })
                        .ok_or_else(|| Box::new(VrmImportError::TextureNotFound(idx as usize)))?;
                }

                if let Some(idx) = texture_properties.emissive {
                    doc.textures(graph)
                        .get(idx as usize)
                        .map(|texture| {
                            material_property.set_emission_map(graph, Some(*texture));
                        })
                        .ok_or_else(|| Box::new(VrmImportError::TextureNotFound(idx as usize)))?;
                }
            }

            let weight = MaterialPropertyWeight {
                name: material_property_json.name,
                float: material_property_json.float.unwrap_or_default(),
                shader: material_property_json.shader,
                vector: material_property_json.vector.unwrap_or_default(),
                tag_map: material_property_json.tag_map.unwrap_or_default(),
                keyword_map: material_property_json.keyword_map.unwrap_or_default(),
                render_queue: material_property_json.render_queue,
            };

            material_property.write(graph, &weight);
        }

        let meta = if let Some(meta) = ext.meta {
            if let Some(idx) = meta.texture {
                doc.textures(graph)
                    .get(idx as usize)
                    .map(|texture| {
                        vrm.set_thumbnail(graph, Some(*texture));
                    })
                    .ok_or_else(|| Box::new(VrmImportError::TextureNotFound(idx as usize)))?;
            }

            Meta {
                title: meta.title,
                version: meta.version,
                author: meta.author,
                reference: meta.reference,
                license_name: meta.license_name,
                allowed_user_name: meta.allowed_user_name,
                sexual_usage_name: meta.sexual_usage_name,
                other_license_url: meta.other_license_url,
                violent_usage_name: meta.violent_usage_name,
                contact_information: meta.contact_information,
                other_permission_url: meta.other_permission_url,
                commercial_usage_name: meta.commercial_usage_name,
            }
        } else {
            Meta::default()
        };

        let mut graph_bones = Vec::new();

        let humanoid = if let Some(humanoid) = ext.humanoid {
            let bones = humanoid.human_bones.unwrap_or_default();

            for bone_json in bones {
                let bone = Bone::new(graph);
                graph_bones.push(bone);
                vrm.add_human_bone(graph, bone);

                if let Some(node_idx) = bone_json.node {
                    doc.nodes(graph)
                        .get(node_idx as usize)
                        .map(|node| {
                            bone.set_node(graph, Some(*node));
                        })
                        .ok_or_else(|| Box::new(VrmImportError::NodeNotFound(node_idx as usize)))?;
                }

                let weight = BoneWeight {
                    name: bone_json.bone,
                    use_default_values: bone_json.use_default_values,
                };

                bone.write(graph, &weight)
            }

            Humanoid {
                arm_stretch: humanoid.arm_stretch,
                leg_stretch: humanoid.leg_stretch,
                feet_spacing: humanoid.feet_spacing,
                upper_arm_twist: humanoid.upper_arm_twist,
                lower_arm_twist: humanoid.lower_arm_twist,
                upper_leg_twist: humanoid.upper_leg_twist,
                lower_leg_twist: humanoid.lower_leg_twist,
                has_translation_dof: humanoid.has_translation_dof,
            }
        } else {
            Humanoid::default()
        };

        let first_person = if let Some(first_person) = ext.first_person {
            if let Some(bone_idx) = first_person.first_person_bone {
                graph_bones
                    .get(bone_idx as usize)
                    .map(|bone| {
                        vrm.set_first_person_bone(graph, Some(*bone));
                    })
                    .ok_or_else(|| Box::new(VrmImportError::BoneNotFound(bone_idx as usize)))?;
            }

            FirstPerson {
                look_at_type_name: first_person.look_at_type_name,
                look_at_vertical_up: first_person.look_at_vertical_up,
                look_at_vertical_down: first_person.look_at_vertical_down,
                first_person_bone_offset: first_person.first_person_bone_offset.unwrap_or_default(),
                look_at_horizontal_inner: first_person.look_at_horizontal_inner,
                look_at_horizontal_outer: first_person.look_at_horizontal_outer,
            }
        } else {
            FirstPerson::default()
        };

        if let Some(blend_shape_master) = ext.blend_shape_master {
            let blend_shape_groups = blend_shape_master.blend_shape_groups.unwrap_or_default();

            for group_json in blend_shape_groups {
                let group = BlendShapeGroup::new(graph);

                let binds = group_json.binds.unwrap_or_default();

                for bind_json in binds {
                    let bind = Bind::new(graph);

                    if let Some(mesh_idx) = bind_json.mesh {
                        if let Some(mesh) = doc.meshes(graph).get(mesh_idx as usize) {
                            let index = bind_json.index.unwrap_or_default();

                            if let Some(primitive) = mesh.primitives(graph).get(index as usize) {
                                bind.set_primitive(graph, Some(*primitive));
                            }
                        }
                    }

                    let weight = BindWeight {
                        weight: bind_json.weight,
                    };

                    bind.write(graph, &weight);
                }

                let weight = BlendShapeGroupWeight {
                    name: group_json.name,
                    preset_name: group_json.preset_name,
                    is_binary: group_json.is_binary,
                    material_values: group_json.material_values.unwrap_or_default(),
                };

                group.write(graph, &weight);
            }
        }

        if let Some(secondary_animation) = ext.secondary_animation {
            let collider_groups = secondary_animation.collider_groups.unwrap_or_default();

            let mut graph_collider_groups = Vec::new();

            for collider_group_json in collider_groups {
                let collider_group = ColliderGroup::new(graph);
                graph_collider_groups.push(collider_group);

                if let Some(node_idx) = collider_group_json.node {
                    doc.nodes(graph)
                        .get(node_idx as usize)
                        .map(|node| {
                            collider_group.set_node(graph, Some(*node));
                        })
                        .ok_or_else(|| Box::new(VrmImportError::NodeNotFound(node_idx as usize)))?;
                }

                let weight = ColliderGroupWeight {
                    colliders: collider_group_json.colliders.unwrap_or_default(),
                };

                collider_group.write(graph, &weight);
            }

            let bone_groups = secondary_animation.bone_groups.unwrap_or_default();

            for bone_group_json in bone_groups {
                let bone_group = BoneGroup::new(graph);
                vrm.add_bone_group(graph, bone_group);

                let bones = bone_group_json.bones.unwrap_or_default();

                for bone_idx in bones {
                    doc.nodes(graph)
                        .get(bone_idx as usize)
                        .map(|node| {
                            bone_group.add_bone(graph, *node);
                        })
                        .ok_or_else(|| Box::new(VrmImportError::BoneNotFound(bone_idx as usize)))?;
                }

                let bone_collider_group_idxs = bone_group_json.collider_groups.unwrap_or_default();

                for collider_group_idx in bone_collider_group_idxs {
                    graph_collider_groups
                        .get(collider_group_idx as usize)
                        .map(|collider_group| {
                            bone_group.add_collider_group(graph, *collider_group);
                        })
                        .ok_or_else(|| {
                            Box::new(VrmImportError::ColliderGroupNotFound(
                                collider_group_idx as usize,
                            ))
                        })?;
                }

                let weight = BoneGroupWeight {
                    center: bone_group_json.center,
                    comment: bone_group_json.comment,
                    stiffiness: bone_group_json.stiffiness,
                    drag_force: bone_group_json.drag_force,
                    hit_radius: bone_group_json.hit_radius,
                    gravity_dir: bone_group_json.gravity_dir.unwrap_or_default(),
                    gravity_power: bone_group_json.gravity_power,
                };

                bone_group.write(graph, &weight);
            }
        }

        let weight = VrmWeight {
            meta,
            humanoid,
            first_person,
            exporter_version: ext.exporter_version.unwrap_or_default(),
        };

        vrm.write(graph, &weight);

        Ok(())
    }
}
