use bevy::prelude::*;
use crate::{SpringBoneLogicState, SpringBones};

pub struct SpringBonePlugin;
impl Plugin for SpringBonePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, do_springbone_logic);
        app.register_type::<SpringBones>();
        app.register_type::<SpringBoneLogicState>();
    }
}

fn do_springbone_logic(
    mut global_transforms: Query<(&mut GlobalTransform, &mut Transform)>,
    spring_boness: Query<&SpringBones>,
    mut spring_bone_logic_states: Query<&mut SpringBoneLogicState>,
    parents: Query<&Parent>,
    time: Res<Time>,
    _names: Query<&Name>,
) {
    for spring_bones in spring_boness.iter() {
        for spring_bone in spring_bones.0.iter() {
            for (_i, bone) in spring_bone.bones.iter().enumerate() {
                let bone: Entity = *bone;
                let (global, _) = global_transforms.get(bone).unwrap();
                let mut spring_bone_logic_state = match spring_bone_logic_states.get_mut(bone) {
                    Ok(spring_bone_logic_state) => spring_bone_logic_state,
                    Err(_) => continue,
                };
                let world_position = *global;

                let parent_entity = parents.get(bone).unwrap().get();

                let parent_world_rotation = global_transforms
                    .get(parent_entity)
                    .unwrap()
                    .0
                    .to_scale_rotation_translation()
                    .1;

                let inertia = (spring_bone_logic_state.current_tail
                    - spring_bone_logic_state.prev_tail)
                    * (1.0 - spring_bone.drag_force);
                let stiffness = time.delta_seconds()
                    * (parent_world_rotation
                        * spring_bone_logic_state.bone_axis
                        * spring_bone.stiffness);
                let external =
                    time.delta_seconds() * spring_bone.gravity_dir * spring_bone.gravity_power;

                let mut next_tail =
                    spring_bone_logic_state.current_tail + inertia + stiffness + external;

                next_tail = world_position.translation()
                    + (next_tail - world_position.translation()).normalize()
                        * spring_bone_logic_state.bone_length;

                spring_bone_logic_state.prev_tail = spring_bone_logic_state.current_tail;
                spring_bone_logic_state.current_tail = next_tail;

                let parent_world_matrix = global_transforms
                    .get(parent_entity)
                    .unwrap()
                    .0
                    .compute_matrix();

                let parent_pos = *global_transforms.get(parent_entity).unwrap().0;

                let to = ((parent_world_matrix * spring_bone_logic_state.initial_local_matrix)
                    .inverse()
                    .transform_point3(next_tail))
                .normalize();

                let (mut global, mut local) = global_transforms.get_mut(bone).unwrap();

                local.rotation = spring_bone_logic_state.initial_local_rotation
                    * Quat::from_rotation_arc(spring_bone_logic_state.bone_axis, to);

                *global = parent_pos.mul_transform(*local);
            }
        }
    }
}
