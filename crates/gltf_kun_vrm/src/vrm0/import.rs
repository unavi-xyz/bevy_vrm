use gltf_kun::{
    extensions::{Extension, ExtensionImport},
    graph::{gltf::GltfDocument, ByteNode, Graph},
    io::format::gltf::GltfFormat,
};
use tracing::warn;

use super::{
    bind::{Bind, BindWeight},
    blend_shape_group::{BlendShapeGroup, BlendShapeGroupWeight},
    bone::{Bone, BoneWeight},
    bone_group::{BoneGroup, BoneGroupWeight},
    collider_group::{ColliderGroup, ColliderGroupWeight},
    weight::{FirstPerson, Humanoid, Meta, VrmWeight},
    Vrm, EXTENSION_NAME,
};

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

        let meta = ext
            .meta
            .map(|meta| {
                if let Some(idx) = meta.texture {
                    if let Some(texture) = doc.textures(graph).get(idx as usize) {
                        vrm.set_thumbnail(graph, Some(*texture));
                    } else {
                        warn!("VRM thumbnail texture not found: {}", idx);
                    }
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
            })
            .unwrap_or_default();

        let mut graph_bones = Vec::new();

        let humanoid = ext
            .humanoid
            .map(|humanoid| {
                let bones = humanoid.human_bones.unwrap_or_default();

                for bone_json in bones {
                    let bone = Bone::new(graph);
                    graph_bones.push(bone);
                    vrm.add_human_bone(graph, bone);

                    if let Some(node_idx) = bone_json.node {
                        if let Some(node) = doc.nodes(graph).get(node_idx as usize) {
                            bone.set_node(graph, Some(*node));
                        } else {
                            warn!("VRM humanoid bone node not found: {}", node_idx);
                        }
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
            })
            .unwrap_or_default();

        let first_person = ext
            .first_person
            .map(|first_person| {
                if let Some(bone_idx) = first_person.first_person_bone {
                    if let Some(bone) = graph_bones.get(bone_idx as usize) {
                        vrm.set_first_person_bone(graph, Some(*bone));
                    } else {
                        warn!("VRM first person bone not found: {}", bone_idx);
                    }
                }

                FirstPerson {
                    look_at_type_name: first_person.look_at_type_name,
                    look_at_vertical_up: first_person.look_at_vertical_up,
                    look_at_vertical_down: first_person.look_at_vertical_down,
                    first_person_bone_offset: first_person
                        .first_person_bone_offset
                        .unwrap_or_default(),
                    look_at_horizontal_inner: first_person.look_at_horizontal_inner,
                    look_at_horizontal_outer: first_person.look_at_horizontal_outer,
                }
            })
            .unwrap_or_default();

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
                    if let Some(node) = doc.nodes(graph).get(node_idx as usize) {
                        collider_group.set_node(graph, Some(*node));
                    } else {
                        warn!("VRM collider group node not found: {}", node_idx);
                    }
                }

                let weight = ColliderGroupWeight {
                    colliders: collider_group_json.colliders.unwrap_or_default(),
                };

                collider_group.write(graph, &weight);
            }

            let bone_groups = secondary_animation.bone_groups.unwrap_or_default();

            for bone_group_json in bone_groups {
                let bone_group = BoneGroup::new(graph);

                let bones = bone_group_json.bones.unwrap_or_default();

                for bone_idx in bones {
                    if let Some(bone) = graph_bones.get(bone_idx as usize) {
                        bone_group.add_bone(graph, *bone);
                    } else {
                        warn!("VRM bone group bone not found: {}", bone_idx);
                    }
                }

                let bone_collider_group_idxs = bone_group_json.collider_groups.unwrap_or_default();

                for collider_group_idx in bone_collider_group_idxs {
                    if let Some(collider_group) =
                        graph_collider_groups.get(collider_group_idx as usize)
                    {
                        bone_group.add_collider_group(graph, *collider_group);
                    } else {
                        warn!(
                            "VRM bone group collider group not found: {}",
                            collider_group_idx
                        );
                    }
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
            material_properties: ext.material_properties.unwrap_or_default(),
        };

        vrm.write(graph, &weight);

        Ok(())
    }
}
