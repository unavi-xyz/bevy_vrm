//! [Bevy](https://bevyengine.org/) plugin implementing the [MToon](https://vrm.dev/en/univrm/shaders/shader_mtoon.html) shader.

use bevy::{asset::load_internal_asset, prelude::*};

mod shader;

use bevy_mod_outline::{OutlineBundle, OutlinePlugin, OutlineVolume};
pub use shader::{MtoonMaterial, OutlineMode};

const SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(0x2d86c40a175b);

#[derive(Default)]
pub struct MtoonPlugin;

impl Plugin for MtoonPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(app, SHADER_HANDLE, "mtoon.wgsl", Shader::from_wgsl);

        app.register_type::<OutlineSync>()
            .add_plugins((OutlinePlugin, MaterialPlugin::<MtoonMaterial>::default()))
            .add_systems(Update, (update_mtoon_shader, add_outline, sync_outline));
    }
}

#[derive(Bundle, Clone, Default)]
pub struct MtoonBundle {
    pub mtoon: Handle<MtoonMaterial>,
    pub outline_sync: OutlineSync,
}

/// Marks a [DirectionalLight] to be used for shading within the MToon shader.
/// Only a single [MtoonSun] is allowed.
#[derive(Component)]
pub struct MtoonSun;

fn update_mtoon_shader(
    mut mtoon: ResMut<Assets<MtoonMaterial>>,
    sun: Query<(&GlobalTransform, &DirectionalLight), With<MtoonSun>>,
) {
    for (_, mtoon) in mtoon.iter_mut() {
        if let Ok((transform, light)) = sun.get_single() {
            mtoon.light_dir = transform.back();
            mtoon.light_color = light.color;
        }
    }
}

/// Syncs an Entity's outline with its [MtoonMaterial].
/// Will add the outline if one is not present.
#[derive(Component, Clone, Default, Reflect)]
#[reflect(Component)]
pub struct OutlineSync;

fn add_outline(
    mut commands: Commands,
    mut entities: Query<Entity, (Without<OutlineVolume>, With<OutlineSync>)>,
) {
    for entity in entities.iter_mut() {
        commands.entity(entity).insert(OutlineBundle::default());
    }
}

fn sync_outline(
    cameras: Query<&GlobalTransform, With<Camera>>,
    materials: Res<Assets<MtoonMaterial>>,
    mut entities: Query<
        (&mut OutlineVolume, &Handle<MtoonMaterial>, &GlobalTransform),
        With<OutlineSync>,
    >,
    windows: Query<&Window>,
) {
    if entities.is_empty() {
        return;
    }

    let max_height = windows
        .iter()
        .fold(0.0f32, |max, window| max.max(window.height()));

    for (mut outline, handle, transform) in entities.iter_mut() {
        let material = match materials.get(handle) {
            Some(m) => m,
            None => continue,
        };

        match material.outline_mode {
            OutlineMode::None => {
                outline.visible = false;
                continue;
            }
            OutlineMode::Screen => {
                outline.visible = true;

                // Outline width is a ratio of screen height.
                outline.width = material.outline_width * max_height;
            }
            OutlineMode::World => {
                outline.visible = true;

                // Outline width is in meters.
                // The meter -> pixel conversion is different for each camera, but we do not get
                // that level of control with the outline shader. So instead, we just approximate
                // the pixel width using the distance from closest camera.
                let distance = cameras
                    .iter()
                    .map(|camera| (camera.translation() - transform.translation()).length())
                    .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                    .unwrap_or_default();

                outline.width = (material.outline_width * max_height * 0.04) / distance;
            }
        }

        outline.colour = material.outline_color;
    }
}
