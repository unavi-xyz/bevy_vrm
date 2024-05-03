use bevy::prelude::*;
use bevy_egui::EguiContexts;

use crate::Settings;

pub fn update_ui(mut contexts: EguiContexts, mut settings: ResMut<Settings>) {
    bevy_egui::egui::Window::new("VRM Viewer").show(contexts.ctx_mut(), |ui| {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;

                ui.label("Loads ");
                ui.hyperlink_to("VRM", "https://vrm.dev/en");
                ui.label(" avatars using ");
                ui.hyperlink_to("bevy_vrm", "https://github.com/unavi-xyz/bevy_vrm");
                ui.label(", a plugin for the ");
                ui.hyperlink_to("Bevy", "https://bevyengine.org");
                ui.label(" game engine.");
            });

            ui.label("Drop a .vrm file into the window to load it.");

            ui.separator();

            ui.checkbox(&mut settings.draw_spring_bones, "Draw spring bones");
            ui.checkbox(&mut settings.move_leg, "Move leg bone");

            ui.separator();

            ui.vertical_centered(|ui| {
                ui.horizontal(|ui| {
                    ui.hyperlink_to("[github]", "https://github.com/unavi-xyz/bevy_vrm");
                });
            });
        });
    });
}
