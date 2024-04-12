use bevy::{asset::AssetMetaCheck, prelude::*};
use vrm_viewer::VrmViewerPlugin;

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins(VrmViewerPlugin)
        .run();
}
