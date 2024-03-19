use bevy::{asset::AssetMetaCheck, prelude::*};
use vrm_viewer::VrmViewerPlugin;

fn main() {
    let mut app = App::new();

    app.insert_resource(AssetMetaCheck::Never)
        .add_plugins((DefaultPlugins, VrmViewerPlugin));

    #[cfg(target_family = "wasm")]
    {
        app.add_plugins(bevy_web_file_drop::WebFileDropPlugin);
    }

    app.run();
}
