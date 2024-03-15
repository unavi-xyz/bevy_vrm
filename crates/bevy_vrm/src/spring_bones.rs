use bevy::{prelude::*, scene::SceneInstance};
use gltf_kun::graph::{
    gltf::{GltfDocument, GltfWeight},
    ByteNode, Extensions, Weight,
};

use crate::{loader::Vrm, SpringBone, SpringBones};

#[derive(Component)]
pub struct SpringBonesInitialized;

pub fn set_spring_bones(
    mut commands: Commands,
    mut vrm: Query<
        (Entity, &mut SpringBones, &Handle<Vrm>, &SceneInstance),
        Without<SpringBonesInitialized>,
    >,
    names: Query<(Entity, &Name)>,
    scene_manager: Res<SceneSpawner>,
    vrms: Res<Assets<Vrm>>,
) {
    for (entity, mut spring_bones, handle, instance) in vrm.iter_mut() {
        if !scene_manager.instance_is_ready(**instance) {
            continue;
        }

        if let Some(vrm) = vrms.get(handle) {
            commands.entity(entity).insert(SpringBonesInitialized);

            let graph = &vrm.gltf.graph;

            let doc = match graph.node_indices().find(|n| {
                let weight = graph.node_weight(*n);
                matches!(weight, Some(Weight::Gltf(GltfWeight::Document)))
            }) {
                Some(doc) => GltfDocument(doc),
                None => continue,
            };

            let ext = match doc.get_extension::<gltf_kun_vrm::vrm0::Vrm>(graph) {
                Some(ext) => ext,
                None => continue,
            };

            for bone_group in ext.bone_groups(graph) {
                let bones = bone_group
                    .bones(graph)
                    .into_iter()
                    .filter_map(|node| {
                        let node_handle = vrm.gltf.node_handles.get(&node).unwrap();

                        let node_name = vrm.gltf.named_nodes.iter().find_map(|(name, node)| {
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

                spring_bones.0.push(SpringBone {
                    bones,
                    center: weight.center.unwrap_or_default(),
                    drag_force: weight.drag_force.unwrap_or_default(),
                    gravity_dir,
                    gravity_power: weight.gravity_power.unwrap_or_default(),
                    hit_radius: weight.hit_radius.unwrap_or_default(),
                    stiffiness: weight.stiffiness.unwrap_or_default(),
                });
            }
        }
    }
}
