use bevy::{
    ecs::component::Component,
    prelude::{
        Vec3,
        World,
    },
};
use bevy_vrm::spring_bones::{
    SpringBone,
    SpringBones,
};

#[test]
fn spring_bones_remap_on_component_map_entities() {
    let mut world = World::new();
    let source = world.spawn_empty().id();
    let target = world.spawn_empty().id();

    let mut spring_bones = SpringBones(vec![SpringBone {
        bones:         vec![source],
        bone_names:    vec!["hair".to_string()],
        center:        0.0,
        drag_force:    0.0,
        gravity_dir:   Vec3::ZERO,
        gravity_power: 0.0,
        hit_radius:    0.0,
        stiffness:     0.0,
    }]);

    Component::map_entities(&mut spring_bones, &mut (source, target));

    assert_eq!(
        spring_bones.0[0].bones[0], target,
        "SpringBones must remap bone entities via Component::map_entities"
    );
}
