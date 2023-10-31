use bevy::prelude::*;

mod extensions;
pub mod loader;

pub struct VrmPlugin;

impl Plugin for VrmPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset_loader::<loader::VrmLoader>();
    }
}
