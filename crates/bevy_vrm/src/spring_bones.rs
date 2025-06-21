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
            .add_systems(
                Update,
                (remap_spring_bone_entities, do_springbone_logic).chain(),
            );
    }
}

fn remap_spring_bone_entities(
    mut spring_bones_query: Query<&mut SpringBones, Added<SpringBones>>,
    names: Query<(Entity, &Name)>,
    existing_entities: Query<Entity>,
) {
    for mut spring_bones in spring_bones_query.iter_mut() {
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
        for spring_bone in spring_bones.0.iter() {
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
                let parent_matrix = parent_global.compute_matrix();
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
