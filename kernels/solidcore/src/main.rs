//! The brep-bench runner for solidcore (https://github.com/tangency-labs/solidcore).
//!
//!     brep-bench-solidcore CASE OUT.stl [OUT.step]
//!     brep-bench-solidcore --version
//!
//! Builds the case `CASE` (`boolean/napkin-ring`) with its implementation,
//! `src/cases/boolean/napkin_ring.rs`, through solidcore-ops, the modelling operations the
//! applications built on solidcore call, then measures the body with the kernel's own tools: its
//! volume and area integrated over the exact B-rep (`Body::volume`, `Body::area`), its faces by
//! surface kind as its record names them, and the kernel's validity check (`Body::check`). It
//! meshes the body with solidcore-tess, solidcore's one mesher, into OUT.stl, writes it as STEP
//! into OUT.step if asked (solidcore's own writer, step-export, through step-brep: see
//! [`write_step`]) and prints one line of JSON. Given OUT.step, the STEP is the whole answer: it is
//! written, and nothing is measured or meshed unless the writer declines. An operation's error is a
//! refusal: the kernel declined, saying why. A case may also refuse in its own words, where
//! solidcore has no operation for what it describes. The runner never reads the case's description.
use solidcore::geom::attribute::AttributeStore;
use solidcore::kernel::{Kernel, UpstreamFixes};
use solidcore::topology::api_files::read_brep_from_file;
use solidcore::topology::brep::p10::TopologyEnv;
use solidcore::topology::loopuse::notify_ref;
use solidcore::topology::topology::NotifyRecords;
use solidcore_ops::{Body, CheckLevel};
use solidcore_record::brep::Surface;
use solidcore_tess::{plan, MeshTolerance};
use std::collections::BTreeMap;
use std::process::ExitCode;

#[allow(dead_code)]
mod bench;
#[allow(dead_code, unused_imports)]
mod shared;

/// A case's implementation: the body it builds, or why it was not built.
type Build = fn() -> Result<Body, Refusal>;

/// Why a case was not built: solidcore's error, in its own words, or a case that refuses in its
/// own because solidcore has no operation for what it describes.
#[derive(Debug)]
pub struct Refusal(String);

impl From<solidcore_ops::Error> for Refusal {
    fn from(e: solidcore_ops::Error) -> Self {
        Refusal(e.to_string())
    }
}

impl From<solidcore::SolidError> for Refusal {
    fn from(e: solidcore::SolidError) -> Self {
        Refusal(e.to_string())
    }
}

impl From<&str> for Refusal {
    fn from(why: &str) -> Self {
        Refusal(why.to_owned())
    }
}

impl From<String> for Refusal {
    fn from(why: String) -> Self {
        Refusal(why)
    }
}

impl std::fmt::Display for Refusal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// One module per case file, and `CASES`, written by `build.rs`.
mod cases {
    use super::{Build, Refusal};
    include!(concat!(env!("OUT_DIR"), "/cases.rs"));
}

/// A face's surface as the reply counts it.
fn kind(s: &Surface) -> &'static str {
    match s {
        Surface::Plane { .. } | Surface::PlaneThrough { .. } => "plane",
        Surface::Cylinder { .. } => "cylinder",
        // a round is recorded as a cone of no angle: a cylinder
        Surface::Cone { half_angle, .. } if *half_angle == 0.0 => "cylinder",
        Surface::Cone { .. } => "cone",
        Surface::Sphere { .. } | Surface::BallPatch { .. } => "sphere",
        Surface::Torus { .. } => "torus",
        Surface::Spline { .. } | Surface::RationalSpline { .. } => "bspline",
        _ => "other",
    }
}

/// A JSON string.
fn quoted(s: &str) -> String {
    let mut out = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn refused(why: &str) -> String {
    format!("{{\"status\":\"refused\",\"reason\":{}}}", quoted(why))
}

/// The triangles written as binary STL.
fn write_stl(path: &str, mesh: &solidcore_tess::Mesh) -> std::io::Result<()> {
    let mut out = Vec::with_capacity(84 + 50 * mesh.triangles.len());
    out.extend_from_slice(&[0u8; 80]);
    out.extend_from_slice(&(mesh.triangles.len() as u32).to_le_bytes());
    for t in &mesh.triangles {
        let [a, b, c] = t.map(|i| mesh.positions[i as usize]);
        let (u, v) = ([b.x - a.x, b.y - a.y, b.z - a.z], [c.x - a.x, c.y - a.y, c.z - a.z]);
        let n = [u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0]];
        let l = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
        let n = if l > 0.0 { n.map(|x| x / l) } else { n };
        for x in [n[0], n[1], n[2], a.x, a.y, a.z, b.x, b.y, b.z, c.x, c.y, c.z] {
            out.extend_from_slice(&(x as f32).to_le_bytes());
        }
        out.extend_from_slice(&[0u8; 2]);
    }
    std::fs::write(path, out)
}

/// The relative accuracies the body is integrated at, finest first: the kernel's finest, 1e-8,
/// does not always converge (a ball's volume), and each measure is taken at the finest that does.
const ACCURACIES: [f64; 5] = [1e-8, 3e-8, 1e-7, 3e-7, 1e-6];

/// A measure at the finest of [`ACCURACIES`] the kernel converges at, or the last error.
fn finest(measure: impl Fn(f64) -> solidcore_ops::Result<f64>) -> solidcore_ops::Result<f64> {
    let mut last = Err(solidcore_ops::Error::Argument);
    for a in ACCURACIES {
        last = measure(a);
        if last.is_ok() {
            break;
        }
    }
    last
}

/// The body written as STEP into `path` with solidcore's own writer, or why not: the writer's
/// words where it declines. The copy of the body is the runner's work, not the writer's, so a
/// failure in it is a crash (the harness's `failed`), not a refusal. ops keeps its kernel body to itself, so the body comes out as its
/// native `.smb` bytes (`Body::to_smb`) and is read back by the kernel as `Body::from_smb` reads
/// them: an exact copy, nothing rebuilt. step-brep maps that B-rep to the neutral STEP model
/// (`brep_to_model`, exact analytic and NURBS geometry), and step-export writes the model as an
/// AP203 Part 21 file (`step_export::write`).
fn write_step(body: &Body, path: &str) -> Result<(), String> {
    let smb = body.to_smb(false).unwrap_or_else(|e| panic!("no .smb copy of the body: {e}"));
    let mut k = Kernel::default();
    k.fixes = UpstreamFixes::all();
    let mut attributes = AttributeStore::default();
    let mut records = NotifyRecords::new();
    let mut env = TopologyEnv {
        all_attributes: &[],
        attributes: &mut attributes,
        key_of: &notify_ref,
        records: &mut records,
        // where ops starts a file's curve and surface keys, clear of the entity keys
        next_key: 1 << 50,
    };
    let mut brep = read_brep_from_file(&mut k, Some(smb), false, false, &mut env)
        .unwrap_or_else(|e| panic!("the .smb copy of the body not read: {e}"));
    brep.refresh_cached_topology();
    let model = step_brep::brep_to_model(&brep).map_err(|e| e.to_string())?;
    let text = step_export::write(&model).map_err(|e| e.to_string())?;
    std::fs::write(path, text).unwrap_or_else(|e| panic!("{path} not written: {e}"));
    Ok(())
}

/// The reply for a body built: its measures, once its triangles are written (and its STEP, if asked
/// for: where the writer declines, the reply says why in `step_error`). The mesh keeps within a
/// ten-thousandth of the body's size, or 1/200 of its thickness (its volume over its area) where
/// that is less: the reference runner's rule (`kernels/occt/run.py`). A STEP written is the whole
/// answer: nothing is then measured or meshed.
fn measured(body: &Body, out: &str, out_step: Option<&str>) -> Result<String, Refusal> {
    // given OUT.step, the solid written there is the whole answer (SPEC.md, "A runner"): nothing
    // measured or meshed; only where the writer declines does the runner go on to its triangles
    let step_error = {
        out_step
            .and_then(|path| write_step(body, path).err())
            .map(|e| format!(",\"step_error\":{}", quoted(&e)))
            .unwrap_or_default()
    };
    if out_step.is_some() && step_error.is_empty() {
        return Ok("{\"status\":\"ok\"}".into());
    }
    let volume = finest(|a| body.volume(a)).map_err(|e| format!("volume not measured: {e}"))?;
    let area = finest(|a| body.area(a)).map_err(|e| format!("area not measured: {e}"))?;
    let valid = body.check(CheckLevel::Geometry).is_ok_and(|r| r.is_valid());
    let (record, _) = body.to_record().map_err(|e| format!("no record: {e}"))?;
    let mut faces = BTreeMap::new();
    for f in &record.faces {
        *faces.entry(kind(&f.surface)).or_insert(0) += 1;
    }
    let faces = faces.iter().map(|(k, n)| format!("\"{k}\":{n}")).collect::<Vec<_>>().join(",");

    let (lo, hi) = body.bounds().map_err(|e| format!("no bounds: {e}"))?;
    let size = ((hi[0] - lo[0]).powi(2) + (hi[1] - lo[1]).powi(2) + (hi[2] - lo[2]).powi(2)).sqrt();
    let thickness = if area > 0.0 && volume > 0.0 { volume / area } else { size };
    let chord = (1e-4 * size).min(5e-3 * thickness).max(1e-7);
    let (tess, stats) =
        plan(&record, &MeshTolerance::chord(chord), size).map_err(|e| format!("not meshed: {e}"))?;
    if let Some(f) = stats.unmeshed.first() {
        return Err(format!("not meshed: face {}: {}", f.face, f.reason).into());
    }
    let (mesh, left_out) = tess.evaluate();
    if let Some(f) = left_out.first() {
        return Err(format!("not meshed: face {}: {}", f.face, f.reason).into());
    }
    write_stl(out, &mesh).map_err(|e| format!("{out} not written: {e}"))?;
    Ok(format!(
        "{{\"status\":\"ok\",\"volume\":{volume:e},\"area\":{area:e},\"faces\":{{{faces}}},\"valid\":{valid}{step_error}}}"
    ))
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some("--version") {
        println!("solidcore {} (brep-bench runner {})", env!("KERNEL_VERSION"), env!("CARGO_PKG_VERSION"));
        return ExitCode::SUCCESS;
    }
    let (case, out, out_step) = match &args[..] {
        [_, case, out] => (case, out, None),
        [_, case, out, step] => (case, out, Some(step.as_str())),
        _ => {
            eprintln!("usage: brep-bench-solidcore CASE OUT.stl [OUT.step]");
            return ExitCode::from(2);
        }
    };
    let Some(&(_, build)) = cases::CASES.iter().find(|(id, _)| id == case) else {
        eprintln!("no implementation of `{case}`");
        return ExitCode::from(2);
    };
    // every fix solidcore makes to the upstream it was ported from: off by default, so that the
    // default path matches SMLib, and on in an application that wants the correct result
    if let Err(e) = solidcore_ops::session::set_fixes(solidcore::kernel::UpstreamFixes::NAMES.iter().copied()) {
        eprintln!("the fixes are not set: {e:?}");
        return ExitCode::from(2);
    }
    let reply = match build().and_then(|body| measured(&body, out, out_step)) {
        Ok(reply) => reply,
        Err(why) => refused(&why.to_string()),
    };
    println!("{reply}");
    ExitCode::SUCCESS
}
