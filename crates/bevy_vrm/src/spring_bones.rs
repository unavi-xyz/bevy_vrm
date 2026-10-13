//! Spring bone components and the simulation driving them.

use bevy::{
    ecs::entity::MapEntities,
    prelude::*,
};

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct SpringBones(#[entities] pub Vec<SpringChain>);

/// Joints ordered so that every parent precedes its children.
#[derive(Reflect)]
pub struct SpringChain {
    pub joints: Vec<SpringJoint>,
}

impl MapEntities for SpringChain {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        for joint in &mut self.joints {
            joint.bone = entity_mapper.get_mapped(joint.bone);
            joint.tail = entity_mapper.get_mapped(joint.tail);
        }
    }
}

/// Simulates `bone` swinging toward `tail`, a descendant of `bone`.
#[derive(Reflect)]
pub struct SpringJoint {
    pub bone:     Entity,
    pub tail:     Entity,
    pub settings: SpringSettings,
}

#[derive(Clone, Copy, Debug, Reflect, PartialEq)]
pub struct SpringSettings {
    pub stiffness:     f32,
    pub drag_force:    f32,
    pub gravity_power: f32,
    pub gravity_dir:   Vec3,
    pub hit_radius:    f32,
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
                (initialize_spring_bone_logic, do_springbone_logic).chain(),
            );
    }
}

/// Position of `tail` in `bone`'s local space, composed from local
/// transforms. `None` unless `bone` is a strict ancestor of `tail`.
fn tail_offset(
    bone: Entity,
    tail: Entity,
    parents: &Query<&ChildOf>,
    local_transforms: &Query<&Transform>,
) -> Option<Vec3> {
    let mut accumulated = Transform::IDENTITY;
    let mut current = tail;

    while current != bone {
        let local = local_transforms.get(current).ok()?;
        accumulated = local.mul_transform(accumulated);
        current = parents.get(current).ok()?.parent();
    }

    (tail != bone).then_some(accumulated.translation)
}

fn initialize_spring_bone_logic(
    global_transforms: Query<&GlobalTransform>,
    local_transforms: Query<&Transform>,
    logic_states: Query<&SpringBoneLogicState>,
    mut commands: Commands,
    parents: Query<&ChildOf>,
    spring_boness: Query<&SpringBones>,
) {
    for spring_bones in spring_boness.iter() {
        for chain in &spring_bones.0 {
            for joint in &chain.joints {
                if logic_states.contains(joint.bone) {
                    continue;
                }

                let Some(offset) = tail_offset(joint.bone, joint.tail, &parents, &local_transforms)
                else {
                    continue;
                };
                if offset == Vec3::ZERO {
                    continue;
                }
                let Ok(global_this_bone) = global_transforms.get(joint.bone) else {
                    continue;
                };
                let Ok(local_this_bone) = local_transforms.get(joint.bone) else {
                    continue;
                };

                let bone_axis = offset.normalize_or_zero();
                let bone_length = offset.length();
                let initial_local_matrix = local_this_bone.to_matrix();
                let initial_local_rotation = local_this_bone.rotation;
                let current_tail = global_this_bone.translation()
                    + (global_this_bone.rotation() * bone_axis * bone_length);

                commands.entity(joint.bone).insert(SpringBoneLogicState {
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

fn do_springbone_logic(
    mut global_transforms: Query<(&mut GlobalTransform, &mut Transform)>,
    mut spring_bone_logic_states: Query<&mut SpringBoneLogicState>,
    parents: Query<&ChildOf>,
    spring_boness: Query<&SpringBones>,
    time: Res<Time>,
) {
    for spring_bones in spring_boness.iter() {
        for chain in &spring_bones.0 {
            for joint in &chain.joints {
                let bone = joint.bone;
                let settings = &joint.settings;

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
                    * (1.0 - settings.drag_force);
                let stiffness = time.delta_secs()
                    * (parent_world_rotation * spring_bone_logic_state.bone_axis)
                    * settings.stiffness;
                let external = time.delta_secs() * settings.gravity_dir * settings.gravity_power;

                let mut next_tail =
                    spring_bone_logic_state.current_tail + inertia + stiffness + external;
                next_tail = global.translation()
                    + (next_tail - global.translation()).normalize_or_zero()
                        * spring_bone_logic_state.bone_length;

                spring_bone_logic_state.prev_tail = spring_bone_logic_state.current_tail;
                spring_bone_logic_state.current_tail = next_tail;

                let Some(to) = (parent_matrix * spring_bone_logic_state.initial_local_matrix)
                    .inverse()
                    .transform_point3(next_tail)
                    .try_normalize()
                else {
                    continue;
                };

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
