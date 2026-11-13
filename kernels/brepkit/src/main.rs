//! The brep-bench runner for brepkit (https://github.com/andymai/brepkit, the `brepkit-*` crates on
//! crates.io).
//!
//!     brep-bench-brepkit CASE OUT.stl [OUT.step]
//!     brep-bench-brepkit --version
//!
//! Builds the case `CASE` (`boolean/napkin-ring`) with its implementation,
//! `src/cases/boolean/napkin_ring.rs`, writes the solid's triangles to `OUT.stl` (and, if asked,
//! the solid to `OUT.step` with brepkit's own STEP writer, `brepkit_io::step::write_step`, which
//! writes AP203) and prints one line of JSON: the kernel's own measures of the solid
//! (`measure::solid_volume` and `measure::solid_surface_area`), its faces by surface kind and
//! whether it passes `validate::validate_solid`, or a refusal. A kernel error is a refusal: the
//! kernel declined, saying why. A case may also refuse in its own words, where the kernel has no
//! operation for what it describes. Given OUT.step, the STEP is the whole answer: it is written,
//! and nothing is measured or meshed unless the writer declines.
//!
//! brepkit measures exactly where it can (planar solids, analytic primitives, solids of planes and
//! quadrics) and otherwise integrates a tessellation of its exact faces; the runner asks it for a
//! fine one. The solid's triangles are `tessellate::tessellate_solid_with_tolerance`, which shares
//! each edge's points between its two faces, so the mesh is welded by the kernel itself.
//!
//! Where brepkit's STEP writer declines a solid, the reply says why in `step_error` and no STEP
//! file is written. The writer serializes each solid's outer shell only (its documentation says
//! so): a solid with voids is written without them.
use brepkit_operations::measure::{solid_bounding_box, solid_surface_area, solid_volume};
use brepkit_operations::tessellate::{tessellate_solid_with_tolerance, TriangleMesh};
use brepkit_operations::validate::validate_solid;
use brepkit_topology::explorer::solid_faces;
use brepkit_topology::face::FaceSurface;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::process::ExitCode;

#[allow(dead_code)]
mod solids;

/// A case's implementation: the solids it builds in the arena, or why they were not built.
type Build = fn(&mut Topology) -> Result<Vec<SolidId>, Refusal>;

/// What a case's `build` returns: one solid, or several separate ones.
pub trait IntoSolids {
    fn into_solids(self) -> Vec<SolidId>;
}

impl IntoSolids for SolidId {
    fn into_solids(self) -> Vec<SolidId> {
        vec![self]
    }
}

impl IntoSolids for Vec<SolidId> {
    fn into_solids(self) -> Vec<SolidId> {
        self
    }
}

/// Why a case was not built: the kernel's error, in its own words, or a case that refuses in its
/// own because the kernel has no operation for what it describes.
#[derive(Debug)]
pub struct Refusal(String);

impl From<brepkit_operations::OperationsError> for Refusal {
    fn from(e: brepkit_operations::OperationsError) -> Self {
        Refusal(e.to_string())
    }
}

impl From<brepkit_topology::TopologyError> for Refusal {
    fn from(e: brepkit_topology::TopologyError) -> Self {
        Refusal(e.to_string())
    }
}

impl From<brepkit_math::MathError> for Refusal {
    fn from(e: brepkit_math::MathError) -> Self {
        Refusal(e.to_string())
    }
}

impl From<brepkit_heal::HealError> for Refusal {
    fn from(e: brepkit_heal::HealError) -> Self {
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
    use super::{Build, IntoSolids, Refusal};
    include!(concat!(env!("OUT_DIR"), "/cases.rs"));
}

/// The kind of a face's surface, in the harness' words.
fn kind(surface: &FaceSurface) -> &'static str {
    match surface {
        FaceSurface::Plane { .. } => "plane",
        FaceSurface::Cylinder(_) => "cylinder",
        FaceSurface::Cone(_) => "cone",
        FaceSurface::Sphere(_) => "sphere",
        FaceSurface::Torus(_) => "torus",
        FaceSurface::Nurbs(_) => "bspline",
    }
}

/// The diagonal of the bounding box of `solids`.
fn extent(topo: &Topology, solids: &[SolidId]) -> Result<f64, Refusal> {
    let mut diag: f64 = 0.0;
    for &s in solids {
        let b = solid_bounding_box(topo, s).map_err(|e| format!("no bounding box: {e}"))?;
        diag = diag.max((b.max - b.min).length());
    }
    Ok(diag)
}

/// The triangles of `mesh` as a binary STL.
fn stl(meshes: &[TriangleMesh]) -> Vec<u8> {
    let count: usize = meshes.iter().map(|m| m.indices.len() / 3).sum();
    let mut out = Vec::with_capacity(84 + 50 * count);
    let mut header = [0u8; 80];
    header[..7].copy_from_slice(b"brepkit");
    out.extend_from_slice(&header);
    out.extend_from_slice(&(count as u32).to_le_bytes());
    for mesh in meshes {
        for t in mesh.indices.chunks_exact(3) {
            let [a, b, c] = [0, 1, 2].map(|i| mesh.positions[t[i] as usize]);
            let n = (b - a).cross(c - a).normalize().unwrap_or(brepkit_math::vec::Vec3::new(0.0, 0.0, 1.0));
            for v in [n.x(), n.y(), n.z()] {
                out.extend_from_slice(&(v as f32).to_le_bytes());
            }
            for p in [a, b, c] {
                for v in [p.x(), p.y(), p.z()] {
                    out.extend_from_slice(&(v as f32).to_le_bytes());
                }
            }
            out.extend_from_slice(&[0, 0]);
        }
    }
    out
}

/// With `out_step`, `solids` written there as STEP; why brepkit would not, if it would not.
fn step(topo: &Topology, solids: &[SolidId], out_step: Option<&str>) -> Option<String> {
    if let Some(path) = out_step {
        match brepkit_io::step::write_step(topo, solids) {
            Ok(step) => std::fs::write(path, step).expect("the STEP file writes"),
            Err(e) => return Some(e.to_string()),
        }
    }
    None
}

/// The measures, face kinds and validity of `solids`, their triangles written to `out` and, if
/// asked, the solids to `out_step` as STEP (`step`). A STEP written is the whole answer: nothing is
/// then measured or meshed.
fn report(
    topo: &Topology,
    solids: &[SolidId],
    out: &str,
    out_step: Option<&str>,
) -> Result<Value, Refusal> {
    if solids.is_empty() {
        return Err("the operation left no solid".into());
    }
    // given OUT.step, the solid written there is the whole answer (SPEC.md, "A runner"): nothing
    // measured or meshed; only where the writer declines does the runner go on to its triangles
    let declined = step(topo, solids, out_step);
    if out_step.is_some() && declined.is_none() {
        return Ok(json!({"status": "ok"}));
    }
    let diag = extent(topo, solids)?;
    // brepkit's measures tessellate what they cannot integrate exactly; ask for a fine one (they
    // clamp it to a small fraction of the solid's extent besides)
    let fine = 1e-6 * diag;
    let (mut volume, mut area, mut valid) = (0.0, 0.0, true);
    let mut faces: BTreeMap<&str, usize> = BTreeMap::new();
    let mut meshes = Vec::new();
    for &s in solids {
        volume += solid_volume(topo, s, fine).map_err(|e| format!("not measured: {e}"))?;
        area += solid_surface_area(topo, s, fine).map_err(|e| format!("not measured: {e}"))?;
        valid &= validate_solid(topo, s).is_ok_and(|r| r.is_valid());
        for f in solid_faces(topo, s)? {
            *faces.entry(kind(topo.face(f)?.surface())).or_default() += 1;
        }
        // a chord of 2e-4 of the extent and 3° of turn: well inside the harness' 2e-2 on the volume
        let mesh = tessellate_solid_with_tolerance(topo, s, 2e-4 * diag, 3f64.to_radians())
            .map_err(|e| format!("not meshed: {e}"))?;
        meshes.push(mesh);
    }
    std::fs::write(out, stl(&meshes)).expect("the STL file writes");
    let mut reply = json!({
        "status": "ok", "volume": volume, "area": area, "faces": faces, "valid": valid,
    });
    if let Some(why) = declined {
        reply["step_error"] = json!(why);
    }
    Ok(reply)
}

fn refused(why: impl std::fmt::Display) -> Value {
    json!({"status": "refused", "reason": why.to_string()})
}

/// Under the harness (`BREP_BENCH_WARM` set; SPEC.md, "A runner"), stops the runner until the
/// harness continues it: once it has loaded and found its case, and once it has answered, so that
/// the harness times the kernel alone, between the two.
#[cfg(any(target_os = "linux", target_os = "macos"))]
fn pause() {
    if std::env::var_os("BREP_BENCH_WARM").is_some() {
        use std::io::Write;
        let _ = std::io::stdout().flush();
        unsafe extern "C" {
            fn raise(signal: std::ffi::c_int) -> std::ffi::c_int;
        }
        #[cfg(target_os = "linux")]
        const SIGSTOP: std::ffi::c_int = 19;
        #[cfg(target_os = "macos")]
        const SIGSTOP: std::ffi::c_int = 17;
        // SAFETY: raise only sends a signal to this process
        unsafe { raise(SIGSTOP) };
    }
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
fn pause() {}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().map(String::as_str) == Some("--version") {
        println!("brepkit {} (runner {})", env!("KERNEL_VERSION"), env!("CARGO_PKG_VERSION"));
        return ExitCode::SUCCESS;
    }
    let (case, out, out_step) = match &args[..] {
        [case, out] => (case, out, None),
        [case, out, step] => (case, out, Some(step.as_str())),
        _ => {
            eprintln!("usage: brep-bench-brepkit CASE OUT.stl [OUT.step] | --version");
            return ExitCode::from(2);
        }
    };
    let Some(&(_, build)) = cases::CASES.iter().find(|(id, _)| id == case) else {
        eprintln!("no implementation of `{case}`");
        return ExitCode::from(2);
    };
    pause();
    let mut topo = Topology::new();
    let reply = match build(&mut topo).and_then(|solids| report(&topo, &solids, out, out_step)) {
        Ok(reply) => reply,
        Err(e) => refused(e),
    };
    println!("{reply}");
    pause();
    ExitCode::SUCCESS
}
