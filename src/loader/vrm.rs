use bevy::asset::LoadContext;

use crate::Vrm;

pub fn load_gltf(
    vrm: &mut Vrm,
    gltf: &goth_gltf::Gltf<super::Extensions>,
    load_context: &mut LoadContext,
) -> Result<(), Box<dyn std::error::Error>> {
    let vrmc_vrm = match &gltf.extensions.vrmc_vrm {
        Some(vrmc_vrm) => vrmc_vrm,
        None => return Err("VRMC_vrm not found".into()),
    };

    Ok(())
}
