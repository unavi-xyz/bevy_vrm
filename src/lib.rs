use bevy::prelude::*;

pub mod loader;

pub struct VRMPlugin;

impl Plugin for VRMPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset_loader::<loader::VRMLoader>();
    }
}
