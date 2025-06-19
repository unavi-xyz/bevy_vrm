use bevy::prelude::*;
use bevy_vrm::spring_bones::SpringBones;

use crate::Settings;

pub fn draw_spring_bones(
    mut gizmos: Gizmos,
    spring_bones: Query<&SpringBones>,
    transforms: Query<&GlobalTransform>,
    settings: Res<Settings>,
) {
    if !settings.draw_spring_bones {
        return;
    }

    for spring_bones in spring_bones.iter() {
        for spring_bone in spring_bones.0.iter() {
            for bone_entity in spring_bone.bones.iter() {
                let transform = match transforms.get(*bone_entity) {
                    Ok(t) => t,
                    Err(_) => {
                        continue;
                    }
                };

                gizmos.sphere(
                    Isometry3d::from_translation(transform.translation()),
                    spring_bone.hit_radius + 0.01,
                    Color::linear_rgb(spring_bone.stiffness, 1.0 - spring_bone.stiffness, 0.1),
                );
            }
        }
    }
}

pub(crate) fn move_avatar(
    mut query: Query<&mut Transform, With<SpringBones>>,
    time: Res<Time>,
    settings: Res<Settings>,
) {
    if !settings.move_avatar {
        return;
    }
    let move_speed = (time.elapsed_secs() + 1.0) / 10.0;
    for mut t in query.iter_mut() {
        let a = time.elapsed_secs() * move_speed;
        let b = a.sin();
        t.rotation.x = b / 20.0;
        t.translation.x += b / 70.0;
    }
}
