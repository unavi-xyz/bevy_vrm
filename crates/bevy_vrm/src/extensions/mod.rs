use bevy::{
    animation::AnimationTarget,
    ecs::system::RunSystemOnce,
    prelude::*,
    transform::systems::{propagate_transforms, sync_simple_transforms},
};
use bevy_gltf_kun::import::{extensions::BevyExtensionImport, gltf::document::ImportContext};
use gltf_kun::{
    extensions::ExtensionImport,
    graph::{
        gltf::{GltfDocument, GltfWeight, Material, Node, Primitive, Scene},
        ByteNode, Edge, Extensions, Graph, Weight,
    },
    io::format::gltf::GltfFormat,
};
use gltf_kun_vrm::vrm0::{
    mesh_annotation::{MeshAnnotation, MeshAnnotationEdges},
    Vrm,
};
use petgraph::{visit::EdgeRef, Direction};
use serde_vrm::vrm0::{BoneName, FirstPersonFlag};

use crate::{
    animations::vrm::VRM_ANIMATION_TARGETS,
    layers::RENDER_LAYERS,
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

        let flag = context
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
            // If mesh is child of the head bone, or a vertex contains the weight of the head bone
            // then change to ThirdPersonOnly. Otherwise, change to Both.
        }

        let layers = RENDER_LAYERS[&flag].clone();
        entity.insert(layers);
    }

    fn import_root(_context: &mut ImportContext) {}

    fn import_scene(context: &mut ImportContext, _scene: Scene, world: &mut World) {
        world.run_system_once(sync_simple_transforms);
        world.run_system_once(propagate_transforms);

        let graph = &context.graph;

        let doc = match graph.node_indices().find(|n| {
            let weight = graph.node_weight(*n);
            matches!(weight, Some(Weight::Gltf(GltfWeight::Document)))
        }) {
            Some(doc) => GltfDocument(doc),
            None => {
                info!("failed to select gltf doc for vrm 0 loading");
                return;
            }
        };

        let ext = match doc.get_extension::<Vrm>(graph) {
            Some(ext) => ext,
            None => {
                info!("failed to select vrm 0 extension for vrm");
                return;
            }
        };

        let names: Vec<(Entity, Name)> = world.run_system_once_with(
            (),
            |names: Query<(Entity, &Name)>| -> Vec<(Entity, Name)> {
                names
                    .iter()
                    .map(|(a, b)| (a, b.clone()))
                    .collect::<Vec<_>>()
            },
        );

        let mut spring_bones = vec![];

        for bone_group in ext.bone_groups(graph) {
            let bones = bone_group
                .bones(graph)
                .into_iter()
                .filter_map(|node| {
                    let node_handle = context.gltf.node_handles.get(&node).unwrap();

                    let node_name = context.gltf.named_nodes.iter().find_map(|(name, node)| {
                        if node == node_handle {
                            Some(name.clone())
                        } else {
                            None
                        }
                    });

                    let node_name = match node_name {
                        Some(name) => name,
                        None => return None,
                    };

                    names.iter().find_map(|(entity, name)| {
                        if name.as_str() == node_name.as_str() {
                            Some(entity)
                        } else {
                            None
                        }
                    })
                })
                .collect::<Vec<_>>();

            let weight = bone_group.read(graph);

            let gravity_dir = Vec3::new(
                weight.gravity_dir.x,
                weight.gravity_dir.y,
                weight.gravity_dir.z,
            );

            spring_bones.push(SpringBone {
                bones: bones.clone().into_iter().copied().collect(),
                center: weight.center.unwrap_or_default(),
                drag_force: weight.drag_force.unwrap_or_default(),
                gravity_dir,
                gravity_power: weight.gravity_power.unwrap_or_default(),
                hit_radius: weight.hit_radius.unwrap_or_default(),
                stiffness: weight.stiffiness.unwrap_or_default(),
            });
        }

        world.run_system_once_with(
            spring_bones,
            |In(spring_bones): In<Vec<SpringBone>>,
             mut commands: Commands,
             query: Query<Entity, Without<Parent>>| {
                commands
                    .entity(query.single())
                    .insert(SpringBones(spring_bones));
            },
        );

        world.run_system_once(
            |mut spring_boness: Query<&mut SpringBones>, children: Query<&Children>| {
                for mut spring_bones in spring_boness.iter_mut() {
                    for spring_bone in spring_bones.0.iter_mut() {
                        let bones = spring_bone.bones.clone();
                        for bone in bones {
                            for child in children.iter_descendants(bone) {
                                if !spring_bone.bones.contains(&child) {
                                    spring_bone.bones.push(child);
                                }
                            }
                        }
                    }
                }
            },
        );

        world.run_system_once(add_springbone_logic_state);

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

            world.run_system_once_with(
                (node_name, bone_name),
                |In((node_name, bone_name)): In<(String, BoneName)>,
                 mut commands: Commands,
                 names: Query<(Entity, &Name)>,
                 parents: Query<&Parent>| {
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
                        root_entity = parent.get();
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
            );
        }
    }
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
                                    TransformBundle {
                                        local: Transform::from_xyz(0.0, -0.07, 0.0),
                                        global: Default::default(),
                                    },
                                    Name::new("donotaddmore"),
                                ))
                                .id();

                            commands.entity(*bone).add_child(child);
                            continue;
                        }
                    };

                    let mut next_bone = None;

                    if let Some(c) = child.iter().next() {
                        next_bone.replace(*c);
                        break;
                    }

                    let next_bone = match next_bone {
                        None => continue,
                        Some(next_bone) => next_bone,
                    };

                    let global_this_bone = global_transforms.get(*bone).unwrap();
                    let local_next_bone = local_transforms.get(next_bone).unwrap();
                    let local_this_bone = local_transforms.get(*bone).unwrap();

                    let bone_axis = local_next_bone.translation.normalize();
                    let bone_length = local_next_bone.translation.length();
                    let initial_local_matrix = local_this_bone.compute_matrix();
                    let initial_local_rotation = local_this_bone.rotation;

                    commands.entity(*bone).insert(SpringBoneLogicState {
                        prev_tail: global_this_bone.translation(),
                        current_tail: global_this_bone.translation(),
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
