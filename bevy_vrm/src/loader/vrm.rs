use bevy::asset::LoadContext;

use crate::Vrm;

pub(crate) fn load_gltf(
    vrm: &mut Vrm,
    goth_gltf: &goth_gltf::Gltf<super::Extensions>,
    _load_context: &mut LoadContext,
) -> Result<(), Box<dyn std::error::Error>> {
    let _vrmc_vrm = match &goth_gltf.extensions.vrmc_vrm {
        Some(vrmc_vrm) => vrmc_vrm,
        None => return Err("VRMC_vrm not found".into()),
    };

    vrm.extensions = goth_gltf.extensions.clone();

    Ok(())
}
