use bevy::{
    animation::AnimationTarget,
    ecs::system::{RunSystemError, RunSystemOnce},
    prelude::*,
    transform::systems::{propagate_parent_transforms, sync_simple_transforms},
};
use bevy_gltf_kun::import::{extensions::BevyExtensionImport, gltf::document::ImportContext};
use gltf_kun::{
    extensions::ExtensionImport,
    graph::{
        ByteNode, Edge, Extensions, Graph, Weight,
        gltf::{GltfDocument, GltfWeight, Material, Node, Primitive, Scene},
    },
    io::format::gltf::GltfFormat,
};
use gltf_kun_vrm::vrm0::{
    Vrm,
    mesh_annotation::{MeshAnnotation, MeshAnnotationEdges},
};
use petgraph::{Direction, visit::EdgeRef};
use serde_vrm::vrm0::{BoneName, FirstPersonFlag};

use crate::{
    animations::vrm::VRM_ANIMATION_TARGETS,
    spring_bones::{SpringBone, SpringBoneLogicState, SpringBones},
};

use self::vrm0::{import_material, import_primitive_material};

pub mod vrm0;
pub mod vrm1;

pub struct VrmExtensions;

impl ExtensionImport<GltfDocument, GltfFormat> for VrmExtensions {
    fn import(
        graph: &mut Graph,
        format: &mut GltfFormat,
        doc: &GltfDocument,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Vrm::import(graph, format, doc)?;

        Ok(())
    }
}

impl BevyExtensionImport<GltfDocument> for VrmExtensions {
    fn import_material(
        context: &mut ImportContext,
        _standard_material: &mut StandardMaterial,
        material: Material,
    ) {
        if let Some(ext) = context.doc.get_extension::<Vrm>(context.graph) {
            import_material(context, material, ext);
        }
    }

    fn import_node(_context: &mut ImportContext, _entity: &mut EntityWorldMut, _node: Node) {}

    fn import_primitive(
        context: &mut ImportContext,
        entity: &mut EntityWorldMut,
        primitive: Primitive,
    ) {
        if let Some(ext) = context.doc.get_extension::<Vrm>(context.graph) {
            import_primitive_material(context, entity, ext, primitive);
        }

        let mut flag = context
            .graph
            .edges_directed(primitive.0, Direction::Incoming)
            .find_map(|edge| {
                if let Edge::Other(name) = edge.weight() {
                    if name == MeshAnnotationEdges::Mesh.to_string().as_str() {
                        let annotation = MeshAnnotation(edge.source());
                        let weight = annotation.read(context.graph);
                        return Some(weight.first_person_flag);
                    }
                }

                None
            })
            .unwrap_or_default();

        if flag == FirstPersonFlag::Auto {
            let mesh = primitive.mesh(context.graph).unwrap();
            let nodes = mesh.nodes(context.graph);

            let Some(ext) = get_vrm_extension(context.graph) else {
                warn!("VRM extension not found");
                return;
            };

            let head = ext
                .human_bones(context.graph)
                .into_iter()
                .find(|b| {
                    let b_weight = b.read(context.graph);
                    b_weight.name == Some(BoneName::Head)
                })
                .unwrap();

            let head_node = head.node(context.graph).unwrap();

            for node in nodes {
                let is_child = find_child(context.graph, node, head_node);

                if is_child {
                    flag = FirstPersonFlag::ThirdPersonOnly;
                    break;
                }
            }
        }

        entity.insert(flag);
    }

    fn import_root(_context: &mut ImportContext) {}

    fn import_scene(context: &mut ImportContext, _scene: Scene, world: &mut World) {
        let _ = world.run_system_once(sync_simple_transforms);
        let _ = world.run_system_once(propagate_parent_transforms);

        let graph = &context.graph;

        let names: Result<Vec<(Entity, Name)>, RunSystemError> =
            world.run_system_once(|names: Query<(Entity, &Name)>| -> Vec<(Entity, Name)> {
                names
                    .iter()
                    .map(|(a, b)| (a, b.clone()))
                    .collect::<Vec<_>>()
            });

        let Ok(names) = names else {
            error!("Error running names system");
            return;
        };

        let Some(ext) = get_vrm_extension(graph) else {
            warn!("VRM extension not found");
            return;
        };

        let mut spring_bones = vec![];

        for bone_group in ext.bone_groups(graph) {
            let (bone_entities, bone_names): (Vec<_>, Vec<_>) = bone_group
                .bones(graph)
                .into_iter()
                .filter_map(|node| {
                    let node_handle = context.gltf.node_handles.get(&node)?;
                    let node_name =
                        context
                            .gltf
                            .named_nodes
                            .iter()
                            .find_map(|(name, gltf_node)| {
                                (gltf_node == node_handle).then(|| name.clone())
                            })?;
                    let (entity, _) = names.iter().find(|(_, name)| name.as_str() == node_name)?;
                    Some((*entity, node_name))
                })
                .unzip();

            let weight = bone_group.read(graph);

            let gravity_dir = Vec3::new(
                weight.gravity_dir.x,
                weight.gravity_dir.y,
                weight.gravity_dir.z,
            );

            spring_bones.push(SpringBone {
                bones: bone_entities,
                bone_names,
                center: weight.center.unwrap_or_default(),
                drag_force: weight.drag_force.unwrap_or_default(),
                gravity_dir,
                gravity_power: weight.gravity_power.unwrap_or_default(),
                hit_radius: weight.hit_radius.unwrap_or_default(),
                stiffness: weight.stiffiness.unwrap_or_default(),
            });
        }

        let _ = world.run_system_once_with(
            |In(spring_bones): In<Vec<SpringBone>>,
             mut commands: Commands,
             query: Query<Entity, Without<ChildOf>>| {
                commands
                    .entity(query.single().unwrap())
                    .insert(SpringBones(spring_bones));
            },
            spring_bones,
        );

        let _ = world.run_system_once(
            |mut spring_boness: Query<&mut SpringBones>,
             children: Query<&Children>,
             names: Query<&Name>| {
                for mut spring_bones in spring_boness.iter_mut() {
                    for spring_bone in spring_bones.0.iter_mut() {
                        let original_bones = spring_bone.bones.clone();
                        for bone in original_bones {
                            for child in children.iter_descendants(bone) {
                                if !spring_bone.bones.contains(&child) {
                                    spring_bone.bones.push(child);
                                    if let Ok(name) = names.get(child) {
                                        spring_bone.bone_names.push(name.to_string());
                                    }
                                }
                            }
                        }
                    }
                }
            },
        );

        let _ = world.run_system_once(add_springbone_logic_state);

        for bone in ext.human_bones(graph) {
            let node = match bone.node(graph) {
                Some(n) => n,
                None => continue,
            };

            let weight = bone.read(graph);

            let bone_name = match weight.name {
                Some(b) => b,
                None => continue,
            };

            let node_handle = match context.gltf.node_handles.get(&node) {
                Some(handle) => handle.clone(),
                None => continue,
            };

            let node_name = context.gltf.named_nodes.iter().find_map(|(name, n)| {
                if *n == node_handle {
                    Some(name.clone())
                } else {
                    None
                }
            });

            let node_name = match node_name {
                Some(n) => n,
                None => continue,
            };

            let _ = world.run_system_once_with(
                |In((node_name, bone_name)): In<(String, BoneName)>,
                 mut commands: Commands,
                 names: Query<(Entity, &Name)>,
                 parents: Query<&ChildOf>| {
                    let node_entity = match names.iter().find_map(|(entity, name)| {
                        if name.as_str() == node_name.as_str() {
                            Some(entity)
                        } else {
                            None
                        }
                    }) {
                        Some(e) => e,
                        None => {
                            warn!("Could not find entity for bone: {}", bone_name);
                            return;
                        }
                    };

                    let mut root_entity = node_entity;
                    while let Ok(parent) = parents.get(root_entity) {
                        root_entity = parent.parent();
                    }

                    commands
                        .entity(root_entity)
                        .insert(AnimationPlayer::default());

                    let id = VRM_ANIMATION_TARGETS[&bone_name];

                    commands.entity(node_entity).insert((
                        AnimationTarget {
                            id,
                            player: root_entity,
                        },
                        bone_name,
                    ));
                },
                (node_name, bone_name),
            );
        }
    }
}

fn find_child(graph: &Graph, target: Node, parent: Node) -> bool {
    if target == parent {
        return true;
    }

    for child in parent.children(graph) {
        if find_child(graph, target, child) {
            return true;
        }
    }

    false
}

fn get_vrm_extension(graph: &Graph) -> Option<Vrm> {
    let doc_idx = graph.node_indices().find(|n| {
        let weight = graph.node_weight(*n);
        matches!(weight, Some(Weight::Gltf(GltfWeight::Document)))
    })?;

    let doc = GltfDocument(doc_idx);

    let ext = doc.get_extension::<Vrm>(graph)?;

    Some(ext)
}

fn add_springbone_logic_state(
    children: Query<&Children>,
    global_transforms: Query<&GlobalTransform>,
    local_transforms: Query<&Transform>,
    logic_states: Query<&mut SpringBoneLogicState>,
    mut commands: Commands,
    names: Query<&Name>,
    spring_boness: Query<(Entity, &SpringBones)>,
) {
    for (_skel_e, spring_bones) in spring_boness.iter() {
        for spring_bone in spring_bones.0.iter() {
            for bone in spring_bone.bones.iter() {
                if !logic_states.contains(*bone) {
                    let child = match children.get(*bone) {
                        Ok(c) => c,
                        Err(_) => {
                            // Adds an extra spring bone below it to make it look even better.
                            if let Ok(name) = names.get(*bone) {
                                if name.as_str() == "donotaddmore" {
                                    continue;
                                }
                            }
                            let child = commands
                                .spawn((
                                    Transform::from_xyz(0.0, -0.07, 0.0),
                                    Name::new("donotaddmore"),
                                ))
                                .id();

                            commands.entity(*bone).add_child(child);
                            continue;
                        }
                    };

                    let Some(next_bone) = child.iter().next() else {
                        continue;
                    };
                    let Ok(global_this_bone) = global_transforms.get(*bone) else {
                        continue;
                    };
                    let Ok(local_next_bone) = local_transforms.get(next_bone) else {
                        continue;
                    };
                    let Ok(local_this_bone) = local_transforms.get(*bone) else {
                        continue;
                    };

                    let bone_axis = local_next_bone.translation.normalize_or_zero();
                    let bone_length = local_next_bone.translation.length();
                    let initial_local_matrix = local_this_bone.compute_matrix();
                    let initial_local_rotation = local_this_bone.rotation;
                    let current_tail = global_this_bone.translation()
                        + (global_this_bone.rotation() * bone_axis * bone_length);

                    commands.entity(*bone).insert(SpringBoneLogicState {
                        prev_tail: current_tail,
                        current_tail,
                        bone_axis,
                        bone_length,
                        initial_local_matrix,
                        initial_local_rotation,
                    });
                }
            }
        }
    }
}
