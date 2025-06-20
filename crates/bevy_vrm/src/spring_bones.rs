use bevy::{
    ecs::{entity::MapEntities, reflect::ReflectMapEntities},
    prelude::*,
};

#[derive(Component, Default, Reflect)]
#[reflect(Component, MapEntities)]
pub struct SpringBones(pub Vec<SpringBone>);

#[derive(Reflect)]
pub struct SpringBone {
    pub bones: Vec<Entity>,
    pub bone_names: Vec<String>,
    pub center: f32,
    pub drag_force: f32,
    pub gravity_dir: Vec3,
    pub gravity_power: f32,
    pub hit_radius: f32,
    pub stiffness: f32,
}

impl MapEntities for SpringBone {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        for bone in &mut self.bones {
            *bone = entity_mapper.get_mapped(*bone);
        }
    }
}

impl MapEntities for SpringBones {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        for bones in &mut self.0 {
            bones.map_entities(entity_mapper);
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct SpringBoneLogicState {
    pub prev_tail: Vec3,
    pub current_tail: Vec3,
    pub bone_axis: Vec3,
    pub bone_length: f32,
    pub initial_local_matrix: Mat4,
    pub initial_local_rotation: Quat,
}

pub struct SpringBonePlugin;

impl Plugin for SpringBonePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SpringBoneLogicState>()
            .register_type::<SpringBones>()
            .add_systems(Update, (remap_spring_bone_entities, do_springbone_logic).chain());
    }
}

fn remap_spring_bone_entities(
    mut spring_bones_query: Query<&mut SpringBones, Added<SpringBones>>,
    names: Query<(Entity, &Name)>,
    existing_entities: Query<Entity>,
) {
    for mut spring_bones in spring_bones_query.iter_mut() {
        let mut needs_remapping = false;
        for spring_bone in spring_bones.0.iter() {
            for &bone_entity in spring_bone.bones.iter() {
                if !existing_entities.contains(bone_entity) {
                    needs_remapping = true;
                    break;
                }
            }
            if needs_remapping {
                break;
            }
        }

        if !needs_remapping {
            continue;
        }

        let mut name_to_entity = std::collections::HashMap::new();
        for (entity, name) in names.iter() {
            name_to_entity.insert(name.as_str(), entity);
        }

        for spring_bone in spring_bones.0.iter_mut() {
            spring_bone.bones.clear();

            for bone_name in &spring_bone.bone_names {
                if let Some(&entity) = name_to_entity.get(bone_name.as_str()) {
                    spring_bone.bones.push(entity);
                }
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
        for spring_bone in spring_bones.0.iter() {
            for bone in spring_bone.bones.iter() {
                let bone: Entity = *bone;
                let (global, _) = match global_transforms.get(bone) {
                    Ok(transforms) => transforms,
                    Err(_) => continue,
                };
                let mut spring_bone_logic_state = match spring_bone_logic_states.get_mut(bone) {
                    Ok(spring_bone_logic_state) => spring_bone_logic_state,
                    Err(_) => continue,
                };
                let world_position = *global;

                let parent_entity = match parents.get(bone) {
                    Ok(parent) => parent.parent(),
                    Err(_) => continue,
                };

                let parent_world_rotation = match global_transforms.get(parent_entity) {
                    Ok((parent_global, _)) => parent_global.to_scale_rotation_translation().1,
                    Err(_) => continue,
                };

                let inertia = (spring_bone_logic_state.current_tail
                    - spring_bone_logic_state.prev_tail)
                    * (1.0 - spring_bone.drag_force);
                let stiffness = time.delta_secs()
                    * (parent_world_rotation
                        * spring_bone_logic_state.bone_axis
                        * spring_bone.stiffness);
                let external =
                    time.delta_secs() * spring_bone.gravity_dir * spring_bone.gravity_power;

                let mut next_tail =
                    spring_bone_logic_state.current_tail + inertia + stiffness + external;

                next_tail = world_position.translation()
                    + (next_tail - world_position.translation()).normalize()
                        * spring_bone_logic_state.bone_length;

                spring_bone_logic_state.prev_tail = spring_bone_logic_state.current_tail;
                spring_bone_logic_state.current_tail = next_tail;

                let (parent_world_matrix, parent_pos) = match global_transforms.get(parent_entity) {
                    Ok((parent_global, _)) => (parent_global.compute_matrix(), *parent_global),
                    Err(_) => continue,
                };

                let to = ((parent_world_matrix * spring_bone_logic_state.initial_local_matrix)
                    .inverse()
                    .transform_point3(next_tail))
                .normalize();

                let (mut global, mut local) = match global_transforms.get_mut(bone) {
                    Ok(transforms) => transforms,
                    Err(_) => continue,
                };

                local.rotation = spring_bone_logic_state.initial_local_rotation
                    * Quat::from_rotation_arc(spring_bone_logic_state.bone_axis, to);

                *global = parent_pos.mul_transform(*local);
            }
        }
    }
}
