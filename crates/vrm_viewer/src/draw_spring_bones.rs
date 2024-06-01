use bevy::prelude::*;
use bevy_vrm::SpringBone;

use crate::Settings;

pub fn draw_spring_bones(
    mut gizmos: Gizmos,
    spring_bones: Query<(&SpringBone, &GlobalTransform)>,
    settings: Res<Settings>,
) {
    if !settings.draw_spring_bones {
        return;
    }

    for (spring_bone, transform) in spring_bones.iter() {
        gizmos.sphere(
            transform.translation(),
            Quat::default(),
            spring_bone.hit_radius,
            Color::rgb(spring_bone.stiffness, 1.0 - spring_bone.stiffness, 0.1),
        );
    }
}
