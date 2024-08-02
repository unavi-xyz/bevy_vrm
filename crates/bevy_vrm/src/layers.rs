use std::sync::LazyLock;

use bevy::{render::view::RenderLayers, utils::HashMap};
use serde_vrm::vrm0::FirstPersonFlag;

pub const FIRST_PERSON_LAYER: usize = 7;
pub const THIRD_PERSON_LAYER: usize = 8;

pub static RENDER_LAYERS: LazyLock<HashMap<FirstPersonFlag, RenderLayers>> = LazyLock::new(|| {
    let mut map = HashMap::default();

    map.insert(
        FirstPersonFlag::Auto,
        RenderLayers::from_layers(&[FIRST_PERSON_LAYER, THIRD_PERSON_LAYER]),
    );
    map.insert(
        FirstPersonFlag::Both,
        RenderLayers::from_layers(&[FIRST_PERSON_LAYER, THIRD_PERSON_LAYER]),
    );
    map.insert(
        FirstPersonFlag::FirstPersonOnly,
        RenderLayers::layer(FIRST_PERSON_LAYER),
    );
    map.insert(
        FirstPersonFlag::ThirdPersonOnly,
        RenderLayers::layer(THIRD_PERSON_LAYER),
    );

    map
});
