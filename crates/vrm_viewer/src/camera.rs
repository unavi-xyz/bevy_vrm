//! Mouse input for Bevy's built-in [`PanOrbitCamera`].
//!
//! Bevy ships the controller without an input plugin. Right drag orbits, left
//! drag pans, the wheel zooms. Motions anchor on the [`MeshPickingPlugin`] hit
//! under the cursor.
//!
//! Shared by the viewer and the workspace examples through `#[path]`.

use bevy::{
    camera_controller::pan_orbit_camera::prelude::{
        DefaultPanOrbitCameraPlugins,
        PanOrbitCamera,
    },
    input::mouse::{
        MouseScrollUnit,
        MouseWheel,
    },
    picking::{
        PickingSystems,
        pointer::{
            PointerAction,
            PointerId,
            PointerInput,
            PointerInteraction,
        },
    },
    prelude::*,
};
use bevy_egui::input::EguiWantsInput;

const ORBIT_BUTTON: MouseButton = MouseButton::Right;
const PAN_BUTTON: MouseButton = MouseButton::Left;
const PIXELS_PER_SCROLL_LINE: f32 = 150.0;

pub struct OrbitCameraPlugin;

impl Plugin for OrbitCameraPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<MeshPickingPlugin>() {
            app.add_plugins(MeshPickingPlugin);
        }
        app.add_plugins(DefaultPanOrbitCameraPlugins).add_systems(
            PreUpdate,
            orbit_camera_input
                .after(PickingSystems::Last)
                .before(PanOrbitCamera::update_camera_positions),
        );
    }
}

fn orbit_camera_input(
    buttons: Res<ButtonInput<MouseButton>>,
    egui: Option<Res<EguiWantsInput>>,
    mut wheel: MessageReader<MouseWheel>,
    mut pointer_inputs: MessageReader<PointerInput>,
    pointers: Query<(&PointerId, &PointerInteraction)>,
    windows: Query<(), With<Window>>,
    mut cameras: Query<(&mut PanOrbitCamera, &GlobalTransform)>,
) {
    let delta = pointer_inputs
        .read()
        .filter(|input| input.pointer_id == PointerId::Mouse)
        .filter_map(|input| match input.action {
            PointerAction::Move { delta } => Some(delta),
            _ => None,
        })
        .sum::<Vec2>();
    let scroll = wheel
        .read()
        .map(|event| match event.unit {
            MouseScrollUnit::Line => event.y * PIXELS_PER_SCROLL_LINE,
            MouseScrollUnit::Pixel => event.y,
        })
        .sum::<f32>();

    // A window hit is in viewport coordinates, not world space.
    let hit = pointers
        .iter()
        .find(|(id, _)| **id == PointerId::Mouse)
        .and_then(|(_, interaction)| interaction.get_nearest_hit())
        .filter(|(entity, _)| !windows.contains(*entity))
        .and_then(|(_, hit)| hit.position);
    let over_ui = egui.is_some_and(|egui| egui.is_pointer_over_area());

    for (mut camera, transform) in &mut cameras {
        if buttons.any_just_released([ORBIT_BUTTON, PAN_BUTTON]) {
            camera.end_move();
        }

        if !over_ui && !camera.is_actively_controlled() {
            let anchor = hit.map(|position| {
                transform
                    .to_matrix()
                    .as_dmat4()
                    .inverse()
                    .transform_point3(position.as_dvec3())
            });
            if buttons.just_pressed(ORBIT_BUTTON) {
                camera.start_orbit(anchor);
            } else if buttons.just_pressed(PAN_BUTTON) {
                camera.start_pan(anchor);
            } else if scroll != 0.0 {
                camera.start_zoom(anchor);
            }
        }

        camera.send_screenspace_input(delta);
        camera.send_zoom_input(scroll);

        let zoom_settled = camera.current_motion.is_zooming_only()
            && camera.current_motion.inputs().is_none_or(|inputs| {
                inputs.zoom_velocity_abs(camera.smoothing.zoom.mul_f32(2.0)) <= 0.0
            });
        if zoom_settled {
            camera.end_move();
        }
    }
}
