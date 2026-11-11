//! The brep-bench runner for popkern (https://github.com/popkern-dev/popkern).
//!
//!     brep-bench-popkern CASE OUT.stl [OUT.step]
//!     brep-bench-popkern --version
//!
//! A popkern part is a Rhai script, so each case is one: `cases/boolean/napkin_ring.rhai` builds
//! `boolean/napkin-ring` the way popkern's users write a part, and ends with its model. The runner
//! runs it in popkern's own script host, takes the model's active body as a part (validated, as
//! popkern serves every part), and reports popkern's own measures of it:
//! `popkern_measure::body_volume` and `body_surface_area`, of the exact faces. Its triangles are
//! popkern's gate tessellation (the mesh `assert_watertight` checks), welded by popkern's own weld.
//! Asked for `OUT.step`, the runner also writes the part there with popkern's own STEP writer,
//! `Part::step_bytes` (what `Part::export_step` writes: `popkern_io::export_step`, AP214), or says
//! in `step_error` why the writer declined it. Given OUT.step, the STEP is the whole answer: it is
//! written, and nothing is measured or meshed unless the writer declines.
//!
//! popkern sorts its own errors: a feature the kernel declines (`FeatureFailed`, `BooleanFailed`,
//! …) is a refusal, in popkern's words. So is a script's own `throw "…"`: a case that refuses,
//! saying what popkern lacks. An error popkern counts as the script's (a syntax error, a bad
//! argument, a selection that matched nothing) is a fault of the implementation, not a refusal:
//! the runner prints it and exits non-zero, so the harness records a crash.
use popkern_assembly::Document;
use popkern_analysis::FaceId;
use popkern_analysis_brep::weld::{WeldedMesh, weld};
use popkern_core::geometry::surface::SurfaceGeometry;
use popkern_part::{Part, PartError};
use popkern_rhai::outcome::{self, Outcome};
use popkern_rhai::{HostOptions, ScriptHost};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

/// What became of a case: built, refused (why), or a fault (what).
enum Failure {
    Refused(String),
    Fault(String),
}

/// The script of `case`: `cases/<category>/<name_snake>.rhai`.
fn script_of(case: &str) -> Option<PathBuf> {
    let (category, name) = case.split_once('/')?;
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("cases")
        .join(category)
        .join(format!("{}.rhai", name.replace('-', "_")));
    path.is_file().then_some(path)
}

/// A failed run, sorted by popkern's own partition of its errors: a feature the kernel declined
/// (`outcome::KERNEL_CLASSES`) or a gate it failed is a refusal, in popkern's words; so is a
/// script's own `throw "…"`. Any other error is the script's: a fault.
fn sort(outcome: Outcome) -> Failure {
    match outcome {
        Outcome::ScriptError { kind, payload: Value::String(why), .. } if kind == "RuntimeError" => {
            Failure::Refused(why)
        }
        Outcome::KernelDecline { rendered, .. } => Failure::Refused(rendered),
        Outcome::GateFailure { step, detail } => Failure::Refused(format!("{step}: {detail}")),
        Outcome::ScriptError { rendered, .. } => Failure::Fault(rendered),
        Outcome::Served => Failure::Fault("a failure that served".into()),
    }
}

fn kind(surface: &SurfaceGeometry) -> &'static str {
    match surface {
        SurfaceGeometry::Plane { .. } => "plane",
        SurfaceGeometry::Cylinder { .. } => "cylinder",
        SurfaceGeometry::Cone { .. } => "cone",
        SurfaceGeometry::Sphere { .. } => "sphere",
        SurfaceGeometry::Torus { .. } => "torus",
        SurfaceGeometry::Nurbs(_) => "bspline",
        _ => "other",
    }
}

/// With `out_step`, `part` written there as STEP, or why popkern's writer would not
/// (`step_error` in `reply`).
fn step(part: &Part, out_step: Option<&Path>, reply: &mut Value) -> Result<(), Failure> {
    if let Some(path) = out_step {
        // the product is named for the file's stem, as `Part::export_step` names it; the writer's
        // refusal is its own error, without the file name `PartError::Io` wraps it in
        let product = path.file_stem().map_or("part".into(), |s| s.to_string_lossy());
        match part.step_bytes(&product) {
            Ok(step) => std::fs::write(path, step)
                .map_err(|e| Failure::Fault(format!("no STEP written: {e}")))?,
            Err(PartError::Io { source, .. }) => reply["step_error"] = json!(source.to_string()),
            Err(e) => reply["step_error"] = json!(e.to_string()),
        }
    }
    Ok(())
}

/// Run the case's script and report the part it built, its triangles written to `out`; with
/// `out_step`, the part written there as STEP (`step`). A STEP written is the whole answer: nothing
/// is then measured or meshed.
fn build(script: &Path, out: &str, out_step: Option<&Path>) -> Result<Value, Failure> {
    let source = std::fs::read_to_string(script)
        .map_err(|e| Failure::Fault(format!("{}: {e}", script.display())))?;
    let name = script.file_name().map_or("case.rhai".into(), |n| n.to_string_lossy());
    let mut host = ScriptHost::with_options(HostOptions::default());
    let mut outcome =
        host.run(&source, &name).map_err(|e| sort(outcome::from_run_error(&e)))?;
    let mut document = outcome
        .take_document()
        .ok_or_else(|| Failure::Fault("the script built no model".into()))?;
    if let Some(failure) = popkern_rhai::document_first_failure(&mut document) {
        return Err(sort(failure));
    }
    // the model's active body, the body its last feature built (or the one `set_active` names):
    // what popkern itself calls the model's part. A body that only carries a path for a sweep
    // stays alive beside it, and is not part of the solid.
    let Document::Part(model) = &mut document else {
        return Err(Failure::Fault("the script built an assembly, not a part".into()));
    };
    let part = model.to_part().map_err(|e| Failure::Refused(e.to_string()))?;
    // given OUT.step, the solid written there is the whole answer (SPEC.md, "A runner"): nothing
    // measured or meshed; only where the writer declines does the runner go on to its triangles
    let mut written = json!({"status": "ok"});
    step(&part, out_step, &mut written)?;
    if out_step.is_some() && written.get("step_error").is_none() {
        return Ok(written);
    }
    let declined = written.get("step_error").cloned();
    let brep = part.brep();
    let tolerance = part.tolerance();
    let volume = popkern_measure::body_volume(brep, &tolerance)
        .map_err(|e| Failure::Refused(format!("no volume: {e}")))?;
    let area = popkern_measure::body_surface_area(brep, &tolerance)
        .map_err(|e| Failure::Refused(format!("no area: {e}")))?;
    let mut faces: BTreeMap<&str, usize> = BTreeMap::new();
    for (_, face) in &brep.faces {
        let k = face.surface.as_ref().map_or("other", |s| kind(&s.geometry));
        *faces.entry(k).or_default() += 1;
    }
    let valid = brep.validate_all().is_empty();
    let mesh = part
        .mesh(&part.gate_tessellation())
        .map_err(|e| Failure::Refused(format!("no tessellation: {e}")))?;
    // popkern meshes each face on its own, sharing no vertices across faces; its own weld (the
    // 1e-7 quantum of its watertightness gate) joins them into one mesh
    let tagged: Vec<_> = (0..mesh.triangles.len()).map(|t| (t, FaceId(0))).collect();
    let welded = weld(&mesh, &tagged).map_err(|e| Failure::Refused(format!("no weld: {e}")))?;
    write_stl(&welded, Path::new(out)).map_err(|e| Failure::Fault(format!("no STL written: {e}")))?;
    let mut reply =
        json!({"status": "ok", "volume": volume, "area": area, "faces": faces, "valid": valid});
    if let Some(why) = declined {
        reply["step_error"] = why;
    }
    Ok(reply)
}

/// `mesh` as a binary STL file.
fn write_stl(mesh: &WeldedMesh, path: &Path) -> std::io::Result<()> {
    let mut bytes = vec![0u8; 80];
    bytes.extend((mesh.triangles.len() as u32).to_le_bytes());
    for t in &mesh.triangles {
        let [a, b, c] = t.map(|i| mesh.vertices[i as usize]);
        let (u, v) = ([0, 1, 2].map(|k| b[k] - a[k]), [0, 1, 2].map(|k| c[k] - a[k]));
        let n = [u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0]];
        let length = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt().max(f64::MIN_POSITIVE);
        for x in n.map(|x| x / length).iter().chain(a.iter()).chain(b.iter()).chain(c.iter()) {
            bytes.extend((*x as f32).to_le_bytes());
        }
        bytes.extend([0, 0]);
    }
    std::fs::write(path, bytes)
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().map(String::as_str) == Some("--version") {
        println!("popkern {} (runner {})", env!("POPKERN_VERSION"), env!("CARGO_PKG_VERSION"));
        return ExitCode::SUCCESS;
    }
    let (case, out, out_step) = match &args[..] {
        [case, out] => (case, out, None),
        [case, out, step] => (case, out, Some(Path::new(step))),
        _ => {
            eprintln!("usage: brep-bench-popkern CASE OUT.stl [OUT.step] | --version");
            return ExitCode::from(2);
        }
    };
    let Some(script) = script_of(case) else {
        eprintln!("no implementation of `{case}`");
        return ExitCode::from(2);
    };
    match build(&script, out, out_step) {
        Ok(reply) => println!("{reply}"),
        Err(Failure::Refused(why)) => println!("{}", json!({"status": "refused", "reason": why})),
        Err(Failure::Fault(what)) => {
            eprintln!("{what}");
            return ExitCode::FAILURE;
        }
    }
    ExitCode::SUCCESS
}
