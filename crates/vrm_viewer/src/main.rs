use bevy::prelude::*;
use vrm_viewer::VrmViewerPlugin;

fn main() {
    App::new().add_plugins(VrmViewerPlugin).run();
}
