//! [Bevy](https://bevyengine.org/) plugin implementing the [MToon](https://vrm.dev/en/univrm/shaders/shader_mtoon.html) shader.

use bevy::{asset::load_internal_asset, prelude::*};

mod shader;

pub use shader::{MtoonMaterial, MtoonShader};

const SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(0x2d86c40a175b);

#[derive(Default)]
pub struct MtoonPlugin;

impl Plugin for MtoonPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(app, SHADER_HANDLE, "mtoon.wgsl", Shader::from_wgsl);

        app.add_plugins(MaterialPlugin::<MtoonMaterial>::default())
            .add_systems(Update, update_mtoon_shader);
    }
}

#[derive(Component)]
pub struct MtoonMainCamera;

#[derive(Component)]
pub struct MtoonSun;

pub fn update_mtoon_shader(
    main_cam: Query<&GlobalTransform, With<MtoonMainCamera>>,
    mut mtoon: ResMut<Assets<MtoonMaterial>>,
    sun: Query<(&GlobalTransform, &DirectionalLight), With<MtoonSun>>,
) {
    for (_, mtoon) in mtoon.iter_mut() {
        if let Ok(cam_t) = main_cam.get_single() {
            mtoon.extension.view_dir = cam_t.back();
        }

        if let Ok((transform, light)) = sun.get_single() {
            mtoon.extension.light_dir = transform.back();
            mtoon.extension.light_color = light.color;
        }
    }
}
