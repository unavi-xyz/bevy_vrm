use bevy::asset::LoadContext;

pub fn load_gltf(
    gltf: &goth_gltf::Gltf<super::Extensions>,
    load_context: &mut LoadContext,
) -> Result<(), Box<dyn std::error::Error>> {
    let vrmc_vrm = gltf.extensions.vrmc_vrm.as_ref();

    if vrmc_vrm.is_none() {
        return Err("VRMC_vrm not found".into());
    }

    let vrmc_vrm = vrmc_vrm.unwrap();

    Ok(())
}
