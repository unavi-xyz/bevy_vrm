use bevy::prelude::*;
use bevy_vrm::BoneName;

use crate::Settings;

pub fn move_leg(
    mut original_transform: Local<Option<Transform>>,
    settings: Res<Settings>,
    time: Res<Time>,
    mut bones: Query<(&mut Transform, &BoneName)>,
) {
    for (mut transform, bone) in bones.iter_mut() {
        if !matches!(bone, BoneName::RightUpperLeg) {
            continue;
        }

        if !settings.move_leg {
            if let Some(original_transform) = original_transform.as_ref() {
                *transform = *original_transform;
            }
            return;
        }

        if original_transform.is_none() {
            *original_transform = Some(*transform);
        }

        let sin = time.elapsed_seconds().sin();
        transform.rotation = Quat::from_rotation_x(sin);
    }
}
