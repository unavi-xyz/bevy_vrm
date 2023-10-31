pub fn load_gltf(
    gltf: &goth_gltf::Gltf<super::Extensions>,
) -> Result<(), Box<dyn std::error::Error>> {
    let vrm0 = gltf.extensions.vrm0.as_ref();

    if vrm0.is_none() {
        return Err("VRM0 not found".into());
    }

    let vrm0 = vrm0.unwrap();

    Ok(())
}
