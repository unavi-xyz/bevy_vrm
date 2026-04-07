use bevy::prelude::*;
use vrm_viewer::VrmViewerPlugin;

// Errors without this, don't know why its needed.
#[cfg(target_arch = "wasm32")]
mod wasm_math {
    #[unsafe(no_mangle)]
    pub extern "C" fn acosh(x: f64) -> f64 {
        libm::acosh(x)
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn acoshf(x: f32) -> f32 {
        libm::acoshf(x)
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn asinh(x: f64) -> f64 {
        libm::asinh(x)
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn asinhf(x: f32) -> f32 {
        libm::asinhf(x)
    }
}

fn main() {
    App::new().add_plugins(VrmViewerPlugin).run();
}
