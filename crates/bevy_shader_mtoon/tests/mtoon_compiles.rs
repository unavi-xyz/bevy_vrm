//! Compiles `mtoon.wesl` and `outline.wesl` with [`wesl`] under the shader-def
//! combinations that bevy's [`MaterialPipeline`] and [`PrepassPipeline`]
//! actually produce, then validates the output with [`naga`]. Imports resolve
//! against the real `bevy_*` shader sources in the cargo registry.
//!
//! [`MaterialPipeline`]: bevy::pbr::MaterialPipeline
//! [`PrepassPipeline`]: bevy::pbr::PrepassPipeline
//!
//! This is a compile-only test: pipeline configurations that fail here would
//! fail at runtime during pipeline specialization (the mesh would not be drawn
//! by that pass). In particular it guards the `PREPASS_PIPELINE` fragment entry
//! points, which compile against `prepass::io::VertexOutput`, a struct that
//! lacks `world_normal` unless `NORMAL_PREPASS_OR_DEFERRED_PREPASS` is set.

use std::{
    borrow::Cow,
    path::{
        Path,
        PathBuf,
    },
};

use wesl::{
    CompileOptions,
    Feature,
    Resolver,
    error::ResolveError,
    syntax::{
        ModulePath,
        PathOrigin,
    },
};

const CRATE_NAME: &str = "bevy_shader_mtoon";

/// Values bevy substitutes into the virtual `constants` module. Exact values
/// are irrelevant for compilation; presence is what matters.
const CONSTANTS: &str = "
const MATERIAL_BIND_GROUP = 3;
const PER_OBJECT_BUFFER_BATCH_SIZE = 4096;
const MAX_RECT_LIGHTS = 64;
const MAX_DIRECTIONAL_LIGHTS = 10;
const MAX_CASCADES_PER_LIGHT = 8;
const SCREEN_SPACE_SPECULAR_TRANSMISSION_BLUR_TAPS = 24;
const TONEMAPPING_LUT_TEXTURE_BINDING_INDEX = 20;
const TONEMAPPING_LUT_SAMPLER_BINDING_INDEX = 21;
";

const MESH_DEFS: &[&str] = &[
    "VIEW_PROJECTION_PERSPECTIVE",
    "VERTEX_OUTPUT_INSTANCE_INDEX",
    "VERTEX_POSITIONS",
    "VERTEX_UVS",
    "VERTEX_UVS_A",
    "VERTEX_NORMALS",
    "VERTEX_TANGENTS",
];

/// Resolves the version `name` is locked to in the workspace `Cargo.lock`, so
/// the test loads the exact crate sources the build compiles against (the
/// registry may also hold older checkouts).
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
/// the version pinned in `Cargo.lock`.
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

/// Mirrors bevy's `embedded_asset!` layout: `crate::a::b` lives at
/// `<crate>/src/a/b.wesl`.
struct EmbeddedResolver;

impl EmbeddedResolver {
    fn file_path(package: &str, path: &ModulePath) -> Result<PathBuf, ResolveError> {
        let root = if package == CRATE_NAME {
            Some(PathBuf::from(env!("CARGO_MANIFEST_DIR")))
        } else {
            registry_crate_dir(package)
        };
        let root = root.ok_or_else(|| {
            ResolveError::ModuleNotFound(path.clone(), format!("{package} not in the registry"))
        })?;
        let mut file = root.join("src");
        file.extend(&path.components);
        Ok(file.with_extension("wesl"))
    }
}

impl Resolver for EmbeddedResolver {
    fn resolve_source<'a>(&'a self, path: &ModulePath) -> Result<Cow<'a, str>, ResolveError> {
        let PathOrigin::Package(package) = &path.origin else {
            return Err(ResolveError::ModuleNotFound(
                path.clone(),
                "expected a package path".to_string(),
            ));
        };
        // wesl qualifies a dependency with its importer (`a/bevy_pbr`), which
        // bevy's resolver strips back to the last segment.
        let package = package.rsplit('/').next().unwrap_or(package);
        if package == "constants" && path.components.is_empty() {
            return Ok(Cow::Borrowed(CONSTANTS));
        }
        let file = Self::file_path(package, path)?;
        std::fs::read_to_string(&file)
            .map(Cow::Owned)
            .map_err(|err| {
                ResolveError::ModuleNotFound(path.clone(), format!("{}: {err}", file.display()))
            })
    }
}

/// Compiles `module` the way bevy's shader cache does, then validates the
/// resulting WGSL.
fn compile(module: &str, defs: &[&str]) -> Result<(), String> {
    let mut options = CompileOptions {
        imports: true,
        condcomp: true,
        visibility: false,
        ..Default::default()
    };
    for def in defs {
        options
            .features
            .flags
            .insert((*def).to_string(), Feature::Enable);
    }

    let main = ModulePath {
        origin:     PathOrigin::Package(CRATE_NAME.to_string()),
        components: vec![module.to_string()],
    };
    let wgsl = wesl::compile(&main, &options, &EmbeddedResolver)
        .map_err(|err| err.diagnostic().render_plain())?
        .to_string();

    let parsed = naga::front::wgsl::parse_str(&wgsl).map_err(|err| err.emit_to_string(&wgsl))?;
    naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::all(),
    )
    .validate(&parsed)
    .map_err(|err| err.emit_to_string(&wgsl))?;
    Ok(())
}

fn with(base: &[&'static str], extra: &[&'static str]) -> Vec<&'static str> {
    [base, extra].concat()
}

#[test]
fn mtoon_compiles_for_all_pipeline_configs() {
    // Main (opaque) pass, matching `MaterialPipeline` shader defs.
    compile("mtoon", MESH_DEFS).expect("main opaque pass compiles");
    compile("mtoon", &with(MESH_DEFS, &["MAY_DISCARD"])).expect("main mask pass compiles");

    // Depth-only prepass (material may discard, no color targets).
    let depth_only = with(
        MESH_DEFS,
        &["PREPASS_PIPELINE", "DEPTH_PREPASS", "MAY_DISCARD"],
    );
    compile("mtoon", &depth_only).expect("depth-only prepass compiles");

    // Depth + motion vectors (TemporalAntiAliasing).
    let taa = with(
        &depth_only,
        &[
            "MOTION_VECTOR_PREPASS",
            "MOTION_VECTOR_PREPASS_OR_DEFERRED_PREPASS",
            "PREPASS_FRAGMENT",
        ],
    );
    compile("mtoon", &taa).expect("TAA prepass compiles");

    // Depth + normals (SSAO, deferred).
    let ssao = with(
        &depth_only,
        &[
            "NORMAL_PREPASS",
            "NORMAL_PREPASS_OR_DEFERRED_PREPASS",
            "PREPASS_FRAGMENT",
        ],
    );
    compile("mtoon", &ssao).expect("normal prepass compiles");

    // Depth + normals + motion vectors.
    let full = with(
        &ssao,
        &[
            "MOTION_VECTOR_PREPASS",
            "MOTION_VECTOR_PREPASS_OR_DEFERRED_PREPASS",
        ],
    );
    compile("mtoon", &full).expect("full prepass compiles");

    // Unclipped depth emulation (WebGL2): fragment writes `frag_depth`.
    let unclipped = with(
        MESH_DEFS,
        &[
            "PREPASS_PIPELINE",
            "DEPTH_PREPASS",
            "PREPASS_FRAGMENT",
            "UNCLIPPED_DEPTH_ORTHO_EMULATION",
        ],
    );
    compile("mtoon", &unclipped).expect("unclipped depth prepass compiles");
}

#[test]
fn outline_compiles_for_mesh_configs() {
    compile("outline", MESH_DEFS).expect("static mesh compiles");
    compile("outline", &with(MESH_DEFS, &["SKINNED"])).expect("skinned mesh compiles");
    compile("outline", &with(MESH_DEFS, &["SKINNED", "MORPH_TARGETS"]))
        .expect("skinned morph mesh compiles");
}
