use bevy::{
    ecs::entity::MapEntities,
    prelude::*,
};

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct SpringBones(#[entities] pub Vec<SpringBone>);

#[derive(Reflect)]
pub struct SpringBone {
    pub bones:         Vec<Entity>,
    pub bone_names:    Vec<String>,
    pub center:        f32,
    pub drag_force:    f32,
    pub gravity_dir:   Vec3,
    pub gravity_power: f32,
    pub hit_radius:    f32,
    pub stiffness:     f32,
}

impl MapEntities for SpringBone {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        for bone in &mut self.bones {
            *bone = entity_mapper.get_mapped(*bone);
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct SpringBoneLogicState {
    pub prev_tail:              Vec3,
    pub current_tail:           Vec3,
    pub bone_axis:              Vec3,
    pub bone_length:            f32,
    pub initial_local_matrix:   Mat4,
    pub initial_local_rotation: Quat,
}

pub struct SpringBonePlugin;

impl Plugin for SpringBonePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SpringBoneLogicState>()
            .register_type::<SpringBones>()
            .add_systems(
                Update,
                (
                    remap_spring_bone_entities,
                    expand_spring_bones,
                    initialize_spring_bone_logic,
                    do_springbone_logic,
                )
                    .chain(),
            );
    }
}

fn expand_spring_bones(
    mut spring_boness: Query<&mut SpringBones>,
    children: Query<&Children>,
    names: Query<&Name>,
) {
    for mut spring_bones in &mut spring_boness {
        for spring_bone in &mut spring_bones.0 {
            for bone in spring_bone.bones.clone() {
                for child in children.iter_descendants(bone) {
                    if names.get(child).is_ok_and(|n| n.as_str() == "donotaddmore") {
                        continue;
                    }
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
}

fn initialize_spring_bone_logic(
    children: Query<&Children>,
    global_transforms: Query<&GlobalTransform>,
    local_transforms: Query<&Transform>,
    logic_states: Query<&SpringBoneLogicState>,
    mut commands: Commands,
    names: Query<&Name>,
    spring_boness: Query<&SpringBones>,
) {
    for spring_bones in spring_boness.iter() {
        for spring_bone in &spring_bones.0 {
            for bone in &spring_bone.bones {
                if logic_states.contains(*bone) {
                    continue;
                }

                let Ok(child) = children.get(*bone) else {
                    if names.get(*bone).is_ok_and(|n| n.as_str() == "donotaddmore") {
                        continue;
                    }
                    let child = commands
                        .spawn((
                            Transform::from_xyz(0.0, -0.07, 0.0),
                            Name::new("donotaddmore"),
                        ))
                        .id();
                    commands.entity(*bone).add_child(child);
                    continue;
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
                let initial_local_matrix = local_this_bone.to_matrix();
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

fn remap_spring_bone_entities(
    mut spring_bones_query: Query<&mut SpringBones, Added<SpringBones>>,
    names: Query<(Entity, &Name)>,
    existing_entities: Query<Entity>,
) {
    for mut spring_bones in &mut spring_bones_query {
        let needs_remapping = spring_bones
            .0
            .iter()
            .flat_map(|spring_bone| &spring_bone.bones)
            .any(|&entity| !existing_entities.contains(entity));

        if !needs_remapping {
            continue;
        }

        let name_to_entity: std::collections::HashMap<&str, Entity> = names
            .iter()
            .map(|(entity, name)| (name.as_str(), entity))
            .collect();

        for spring_bone in &mut spring_bones.0 {
            spring_bone.bones = spring_bone
                .bone_names
                .iter()
                .filter_map(|name| name_to_entity.get(name.as_str()).copied())
                .collect();
        }
    }
}

fn do_springbone_logic(
    mut global_transforms: Query<(&mut GlobalTransform, &mut Transform)>,
    mut spring_bone_logic_states: Query<&mut SpringBoneLogicState>,
    parents: Query<&ChildOf>,
    spring_boness: Query<&SpringBones>,
    time: Res<Time>,
) {
    for spring_bones in spring_boness.iter() {
        for spring_bone in &spring_bones.0 {
            for &bone in &spring_bone.bones {
                let Ok((global, _)) = global_transforms.get(bone) else {
                    continue;
                };
                let Ok(mut spring_bone_logic_state) = spring_bone_logic_states.get_mut(bone) else {
                    continue;
                };
                let Ok(parent) = parents.get(bone) else {
                    continue;
                };
                let parent_entity = parent.parent();

                let Ok((parent_global, _)) = global_transforms.get(parent_entity) else {
                    continue;
                };
                let parent_world_rotation = parent_global.to_scale_rotation_translation().1;
                let parent_matrix = parent_global.to_matrix();
                let parent_global_transform = *parent_global;

                let inertia = (spring_bone_logic_state.current_tail
                    - spring_bone_logic_state.prev_tail)
                    * (1.0 - spring_bone.drag_force);
                let stiffness = time.delta_secs()
                    * (parent_world_rotation * spring_bone_logic_state.bone_axis)
                    * spring_bone.stiffness;
                let external =
                    time.delta_secs() * spring_bone.gravity_dir * spring_bone.gravity_power;

                let mut next_tail =
                    spring_bone_logic_state.current_tail + inertia + stiffness + external;
                next_tail = global.translation()
                    + (next_tail - global.translation()).normalize()
                        * spring_bone_logic_state.bone_length;

                spring_bone_logic_state.prev_tail = spring_bone_logic_state.current_tail;
                spring_bone_logic_state.current_tail = next_tail;

                let to = ((parent_matrix * spring_bone_logic_state.initial_local_matrix)
                    .inverse()
                    .transform_point3(next_tail))
                .normalize();

                let Ok((mut global, mut local)) = global_transforms.get_mut(bone) else {
                    continue;
                };
                local.rotation = spring_bone_logic_state.initial_local_rotation
                    * Quat::from_rotation_arc(spring_bone_logic_state.bone_axis, to);
                *global = parent_global_transform.mul_transform(*local);
            }
        }
    }
}
