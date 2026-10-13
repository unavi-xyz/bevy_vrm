//! Compiles `mtoon.wgsl` with [`naga_oil`] under the shader-def combinations
//! that bevy's [`MaterialPipeline`] and [`PrepassPipeline`] actually produce,
//! using the real `bevy_pbr` / `bevy_render` wgsl modules from the cargo
//! registry.
//!
//! [`MaterialPipeline`]: bevy::pbr::MaterialPipeline
//! [`PrepassPipeline`]: bevy::pbr::PrepassPipeline
//!
//! This is a compile-only test: pipeline configurations that fail to parse here
//! would fail at runtime during pipeline specialization (the mesh would not be
//! drawn by that pass). In particular it guards the `PREPASS_PIPELINE` fragment
//! entry points, which compile against `prepass_io::VertexOutput` — a struct
//! that lacks `world_normal` unless `NORMAL_PREPASS_OR_DEFERRED_PREPASS` is
//! set.

use std::{
    collections::HashMap,
    path::{
        Path,
        PathBuf,
    },
};

use naga_oil::compose::{
    ComposableModuleDescriptor,
    Composer,
    NagaModuleDescriptor,
    ShaderDefValue,
    ShaderLanguage,
    ShaderType,
};

const MTOON_WGSL: &str = include_str!("../src/mtoon.wgsl");

/// Resolves the version `name` is locked to in the workspace `Cargo.lock`, so
/// the test loads the exact crate sources the build compiles against (the lock
/// may hold e.g. `bevy_pbr-0.19.1` while an older checkout sits in the
/// registry).
fn locked_version(name: &str) -> Option<String> {
    let lock =
        std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../Cargo.lock"))
            .ok()?;
    let mut lines = lock.lines();
    while let Some(line) = lines.next() {
        if line == format!("name = \"{name}\"") {
            let version = lines.next()?.strip_prefix("version = \"")?;
            return version.strip_suffix('"').map(str::to_string);
        }
    }
    None
}

/// Finds `<cargo home>/registry/src/*/<name>-<version>` for the `name` crate at
/// the version pinned in `Cargo.lock`. These provide the real shader modules
/// that `mtoon.wgsl` imports.
fn registry_crate_dir(name: &str) -> Option<PathBuf> {
    let dir = format!("{name}-{}", locked_version(name)?);
    let cargo_home = std::env::var_os("CARGO_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".cargo")))?;
    let registry_src = cargo_home.join("registry").join("src");
    for hash_dir in std::fs::read_dir(registry_src).ok()?.flatten() {
        if hash_dir.path().join(&dir).is_dir() {
            return Some(hash_dir.path().join(&dir));
        }
    }
    None
}

/// Recursively collects `(module_name, source)` for every wgsl module with a
/// `#define_import_path` under `root`.
fn collect_modules(root: &Path) -> Vec<(String, String)> {
    let mut modules = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension() == Some(std::ffi::OsStr::new("wgsl")) {
                let source = std::fs::read_to_string(&path).expect("read wgsl source");
                let Some(import_path) = source
                    .lines()
                    .find_map(|line| line.strip_prefix("#define_import_path "))
                else {
                    continue;
                };
                modules.push((import_path.trim().to_string(), source));
            }
        }
    }
    modules
}

/// A composer populated with every `bevy_pbr` and `bevy_render` wgsl module, or
/// `None` if the registry sources cannot be found (offline sandboxes).
fn load_composer() -> Option<Composer> {
    let bevy_pbr = registry_crate_dir("bevy_pbr")?;
    let bevy_render = registry_crate_dir("bevy_render")?;
    let bevy_core_pipeline = registry_crate_dir("bevy_core_pipeline")?;
    let mut modules = collect_modules(&bevy_pbr);
    modules.extend(collect_modules(&bevy_render));
    modules.extend(collect_modules(&bevy_core_pipeline));

    let mut composer = Composer::default().with_capabilities(naga::valid::Capabilities::all());
    let module_count = modules.len();
    let mut unresolved = modules;
    let mut last_errors: std::collections::HashMap<String, String> =
        std::collections::HashMap::new();

    // Modules must be registered in dependency order; keep retrying until no
    // progress is made.
    loop {
        let mut progress = false;
        let mut retry = Vec::new();
        for (name, source) in unresolved {
            let desc = ComposableModuleDescriptor {
                source:             &source,
                file_path:          &name,
                language:           ShaderLanguage::Wgsl,
                as_name:            Some(name.clone()),
                additional_imports: &[],
                shader_defs:        HashMap::new(),
            };
            match composer.add_composable_module(desc) {
                Ok(_) => progress = true,
                Err(err) => {
                    last_errors.insert(name.clone(), format!("{err}"));
                    retry.push((name, source));
                }
            }
        }
        unresolved = retry;
        if !progress {
            break;
        }
    }

    if !unresolved.is_empty() {
        let names: Vec<_> = unresolved.iter().map(|(n, _)| n.as_str()).collect();
        eprintln!("{} modules failed to register: {names:?}", unresolved.len());
        for name in names {
            if let Some(err) = last_errors.get(name) {
                eprintln!("{name}: {}", err.chars().take(200).collect::<String>());
            }
        }
        return None;
    }
    eprintln!(
        "registered {module_count} bevy shader modules ({} bevy_pbr, {} bevy_render)",
        collect_modules(&bevy_pbr).len(),
        collect_modules(&bevy_render).len()
    );
    Some(composer)
}

/// Numeric placeholders bevy substitutes into `@{#...}` bind groups. Exact
/// values are irrelevant for compilation; presence is what matters.
fn base_defs(defs: &[(&str, ShaderDefValue)]) -> HashMap<String, ShaderDefValue> {
    let mut shader_defs: HashMap<String, ShaderDefValue> = [
        ("MATERIAL_BIND_GROUP".to_string(), ShaderDefValue::UInt(3)),
        (
            "PER_OBJECT_BUFFER_BATCH_SIZE".to_string(),
            ShaderDefValue::UInt(4096),
        ),
        ("MAX_RECT_LIGHTS".to_string(), ShaderDefValue::UInt(64)),
        (
            "MAX_DIRECTIONAL_LIGHTS".to_string(),
            ShaderDefValue::UInt(10),
        ),
        (
            "MAX_CASCADES_PER_LIGHT".to_string(),
            ShaderDefValue::UInt(8),
        ),
        (
            "SCREEN_SPACE_SPECULAR_TRANSMISSION_BLUR_TAPS".to_string(),
            ShaderDefValue::UInt(24),
        ),
        (
            "AVAILABLE_STORAGE_BUFFER_BINDINGS".to_string(),
            ShaderDefValue::UInt(8),
        ),
    ]
    .into_iter()
    .collect();
    for (name, value) in defs {
        shader_defs.insert(name.to_string(), *value);
    }
    shader_defs
}

fn vertex_mesh_defs() -> Vec<(&'static str, ShaderDefValue)> {
    [
        ("VIEW_PROJECTION_PERSPECTIVE", ShaderDefValue::Bool(true)),
        ("VERTEX_OUTPUT_INSTANCE_INDEX", ShaderDefValue::Bool(true)),
        ("VERTEX_POSITIONS", ShaderDefValue::Bool(true)),
        ("VERTEX_UVS", ShaderDefValue::Bool(true)),
        ("VERTEX_UVS_A", ShaderDefValue::Bool(true)),
        ("VERTEX_NORMALS", ShaderDefValue::Bool(true)),
        ("VERTEX_TANGENTS", ShaderDefValue::Bool(true)),
    ]
    .into_iter()
    .collect()
}

fn compose(composer: &mut Composer, defs: &[(&str, ShaderDefValue)]) -> Result<(), String> {
    composer
        .make_naga_module(NagaModuleDescriptor {
            source:             MTOON_WGSL,
            file_path:          "bevy_shader_mtoon::mtoon.wgsl",
            shader_type:        ShaderType::Wgsl,
            shader_defs:        base_defs(defs),
            additional_imports: &[],
        })
        .map(|_| ())
        .map_err(|err| format!("{err}"))
}

const fn def(name: &'static str) -> (&'static str, ShaderDefValue) {
    (name, ShaderDefValue::Bool(true))
}

#[test]
fn mtoon_compiles_for_all_pipeline_configs() {
    let mut composer =
        load_composer().expect("could not load bevy shader modules for mtoon compile test");

    let mesh = vertex_mesh_defs();

    // Main (opaque) pass, matching `MaterialPipeline` shader defs.
    compose(&mut composer, &mesh).expect("main opaque pass compiles");
    compose(
        &mut composer,
        &[&mesh[..], &[def("MAY_DISCARD")][..]].concat(),
    )
    .expect("main mask pass compiles");

    // Depth-only prepass (material may discard, no color targets).
    let depth_only = [
        &mesh[..],
        &[
            def("PREPASS_PIPELINE"),
            def("DEPTH_PREPASS"),
            def("MAY_DISCARD"),
        ][..],
    ]
    .concat();
    compose(&mut composer, &depth_only).expect("depth-only prepass compiles");

    // Depth + motion vectors (TemporalAntiAliasing).
    let taa = [
        &depth_only[..],
        &[
            def("MOTION_VECTOR_PREPASS"),
            def("MOTION_VECTOR_PREPASS_OR_DEFERRED_PREPASS"),
            def("PREPASS_FRAGMENT"),
        ][..],
    ]
    .concat();
    compose(&mut composer, &taa).expect("TAA prepass compiles");

    // Depth + normals (SSAO, deferred).
    let ssa_o = [
        &depth_only[..],
        &[
            def("NORMAL_PREPASS"),
            def("NORMAL_PREPASS_OR_DEFERRED_PREPASS"),
            def("PREPASS_FRAGMENT"),
        ][..],
    ]
    .concat();
    compose(&mut composer, &ssa_o).expect("normal prepass compiles");

    // Depth + normals + motion vectors.
    let full = [
        &ssa_o[..],
        &[def("MOTION_VECTOR_PREPASS_OR_DEFERRED_PREPASS")][..],
    ]
    .concat();
    compose(&mut composer, &full).expect("full prepass compiles");

    // Unclipped depth emulation (WebGL2): fragment writes `frag_depth`.
    let unclipped = [
        &mesh[..],
        &[
            def("PREPASS_PIPELINE"),
            def("DEPTH_PREPASS"),
            def("PREPASS_FRAGMENT"),
            def("UNCLIPPED_DEPTH_ORTHO_EMULATION"),
        ][..],
    ]
    .concat();
    compose(&mut composer, &unclipped).expect("unclipped depth prepass compiles");
}
