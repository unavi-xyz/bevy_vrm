use serde_vrm::vrm0::{
    BoneName,
    Vrm,
};

fn load(path: &str) -> Vrm {
    let bytes = std::fs::read(path).expect("read vrm");
    let gltf = gltf::Gltf::from_slice(&bytes).expect("parse glb");
    let value = gltf
        .extensions()
        .and_then(|e| e.get("VRM"))
        .expect("VRM extension present")
        .clone();
    serde_json::from_value::<Vrm>(value).expect("deserialize VRM")
}

#[test]
fn alicia_has_expected_data() {
    let vrm = load("../../assets/alicia.vrm");

    let bone_groups = vrm
        .secondary_animation
        .as_ref()
        .and_then(|s| s.bone_groups.as_ref());
    let group_count = bone_groups.map_or(0, Vec::len);
    let bone_count: usize = bone_groups
        .map(|gs| {
            gs.iter()
                .map(|g| g.bones.as_ref().map_or(0, Vec::len))
                .sum()
        })
        .unwrap_or_default();

    let human_bones = vrm.humanoid.as_ref().and_then(|h| h.human_bones.as_ref());
    let human_bone_count = human_bones.map_or(0, Vec::len);

    let head = human_bones
        .and_then(|bones| bones.iter().find(|b| b.bone == Some(BoneName::Head)))
        .and_then(|b| b.node);

    let annotations = vrm
        .first_person
        .as_ref()
        .and_then(|f| f.mesh_annotations.as_ref())
        .map_or(0, Vec::len);

    eprintln!(
        "alicia: spring_groups={group_count} spring_bones={bone_count} human_bones={human_bone_count} head_node={head:?} mesh_annotations={annotations}"
    );

    assert!(group_count > 0, "expected spring bone groups");
    assert!(human_bone_count > 0, "expected human bones");
    assert!(head.is_some(), "expected head bone");
}
