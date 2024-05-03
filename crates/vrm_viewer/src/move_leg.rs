use bevy::prelude::*;
use bevy_vrm::{BoneName, HumanoidBones};

use crate::Settings;

pub fn move_leg(
    mut original_transform: Local<Option<Transform>>,
    mut transforms: Query<&mut Transform>,
    settings: Res<Settings>,
    time: Res<Time>,
    vrm: Query<&HumanoidBones>,
) {
    for humanoid in vrm.iter() {
        let leg = match humanoid.0.get(&BoneName::RightUpperLeg) {
            Some(leg) => leg,
            None => continue,
        };

        if !settings.move_leg {
            if let Some(original_transform) = original_transform.as_ref() {
                if let Ok(mut transform) = transforms.get_mut(*leg) {
                    *transform = *original_transform;
                }
            }
            return;
        }

        if let Ok(mut transform) = transforms.get_mut(*leg) {
            if original_transform.is_none() {
                *original_transform = Some(*transform);
            }

            let sin = time.elapsed_seconds().sin();
            transform.rotation = Quat::from_rotation_x(sin);
        }
    }
}
