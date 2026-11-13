//! The brep-bench runner for vcad (https://github.com/ecto/vcad).
//!
//!     brep-bench-vcad CASE OUT.stl [OUT.step]
//!     brep-bench-vcad --version
//!
//! Builds the case `CASE` (`boolean/napkin-ring`) with its implementation,
//! `src/cases/boolean/napkin_ring.rs`, writes the solid's triangles to `OUT.stl` (and, if asked,
//! the solid to `OUT.step` with vcad's own STEP writer, `Solid::to_step_buffer`) and prints one
//! line of JSON, or a refusal. Given OUT.step, the STEP is the whole answer: it is written, and
//! nothing is measured or meshed unless the writer declines.
//!
//! vcad measures only meshes: `Solid::volume` and `Solid::surface_area` are its tessellation's.
//! So a built case answers `"measured": "mesh"` and the harness takes the volume and area from the
//! triangles: vcad's own tessellation of the solid (`Solid::to_mesh`, which also closes what its
//! splitters left open), written by vcad's own STL writer. A vcad error is a refusal: the kernel
//! declined, saying why. So is a solid whose history vcad itself records as the wrong geometry (a
//! cut it skipped, meshes it concatenated). A case may also refuse in its own words, where vcad
//! has no operation for what it describes.
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::process::ExitCode;
use vcad_kernel::vcad_kernel_geom::SurfaceKind;
use vcad_kernel::{Solid, SolidFidelity};

mod solids;

/// The segments round a full circle of the written mesh. vcad's error in a curved face's volume
/// and area falls as the square of it: at this one a ball is within 3e-4 of both, inside the 1e-3
/// the harness asks.
const MESH_SEGMENTS: u32 = 256;

/// A case's implementation: the solid it builds, or why it was not built.
type Build = fn() -> Result<Solid, Refusal>;

/// Why a case was not built: a vcad error, in vcad's own words, or a case that refuses in its own
/// because vcad has no operation for what it describes.
#[derive(Debug)]
pub struct Refusal(String);

/// Each vcad error is a refusal in its own words.
macro_rules! refusals {
    ($($error:ty),* $(,)?) => {$(
        impl From<$error> for Refusal {
            fn from(e: $error) -> Self {
                Refusal(e.to_string())
            }
        }
    )*};
}

refusals!(
    vcad_kernel::BooleanError,
    vcad_kernel::BlendError,
    vcad_kernel::vcad_kernel_sketch::SketchError,
    vcad_kernel::vcad_kernel_sweep::SweepError,
    vcad_kernel::vcad_kernel_sweep::LoftError,
);

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

fn kind(k: SurfaceKind) -> &'static str {
    match k {
        SurfaceKind::Plane => "plane",
        SurfaceKind::Cylinder => "cylinder",
        SurfaceKind::Cone => "cone",
        SurfaceKind::Sphere => "sphere",
        SurfaceKind::Torus => "torus",
        SurfaceKind::BSpline => "bspline",
        SurfaceKind::Bilinear => "other",
    }
}

/// With `out_step`, `solid` written there as STEP; why vcad would not, if it would not.
fn step(solid: &Solid, out_step: Option<&str>) -> Option<String> {
    if let Some(path) = out_step {
        match solid.to_step_buffer() {
            Ok(step) => std::fs::write(path, step).expect("the STEP file writes"),
            Err(e) => return Some(e.to_string()),
        }
    }
    None
}

/// The faces of `solid` by surface kind, its validity, and its triangles written to `out`; with
/// `out_step`, the solid written there as STEP (`step`). A STEP written is the whole answer:
/// nothing is then measured or meshed.
fn report(solid: &Solid, out: &str, out_step: Option<&str>) -> Result<Value, Refusal> {
    // vcad's ledger of the solid's construction: an operation that could not be done and left the
    // wrong solid in its place (a skipped cut, two meshes concatenated) is the kernel declining
    if let Some(event) = solid.wrong_geometry_events().next() {
        return Err(Refusal(event.to_string()));
    }
    // given OUT.step, the solid written there is the whole answer (SPEC.md, "A runner"): nothing
    // measured or meshed; only where the writer declines does the runner go on to its triangles
    let declined = step(solid, out_step);
    if out_step.is_some() && declined.is_none() {
        return Ok(json!({"status": "ok"}));
    }
    let mut faces: BTreeMap<&str, usize> = BTreeMap::new();
    if let Some(brep) = solid.brep() {
        let body = &brep.topology.solids[brep.solid_id];
        for &shell in std::iter::once(&body.outer_shell).chain(&body.void_shells) {
            for &f in &brep.topology.shells[shell].faces {
                let surface = &brep.geometry.surfaces[brep.topology.faces[f].surface_index];
                *faces.entry(kind(surface.surface_type())).or_default() += 1;
            }
        }
    }
    // vcad has no checker of a solid's topology; `valid` is its own account of the solid, that it
    // is still a true B-rep, and not the triangles of a boolean that fell back to meshes
    let valid = solid.fidelity() == SolidFidelity::Analytic;
    let mesh = solid.to_mesh(MESH_SEGMENTS);
    if mesh.num_triangles() == 0 {
        return Err("the solid is empty".into());
    }
    let spec = json!({
        "name": "vcad",
        "meshes": [{"positions": [0, mesh.vertices.len()], "indices": [0, mesh.indices.len()]}],
    });
    let bytes = vcad_kernel_export::build_stl_json(&spec.to_string(), &mesh.vertices, &mesh.indices)
        .map_err(|e| Refusal(format!("no STL written: {e:?}")))?;
    std::fs::write(out, bytes).expect("the STL file writes");
    let mut reply = json!({"status": "ok", "measured": "mesh", "faces": faces, "valid": valid});
    if let Some(why) = declined {
        reply["step_error"] = json!(why);
    }
    Ok(reply)
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
        println!("vcad-kernel {} (runner {})", env!("VCAD_VERSION"), env!("CARGO_PKG_VERSION"));
        return ExitCode::SUCCESS;
    }
    let (case, out, out_step) = match &args[..] {
        [case, out] => (case, out, None),
        [case, out, step] => (case, out, Some(step.as_str())),
        _ => {
            eprintln!("usage: brep-bench-vcad CASE OUT.stl [OUT.step] | --version");
            return ExitCode::from(2);
        }
    };
    let Some(&(_, build)) = cases::CASES.iter().find(|(id, _)| id == case) else {
        eprintln!("no implementation of `{case}`");
        return ExitCode::from(2);
    };
    pause();
    let reply = match build().and_then(|solid| report(&solid, out, out_step)) {
        Ok(reply) => reply,
        Err(why) => json!({"status": "refused", "reason": why.to_string()}),
    };
    println!("{reply}");
    pause();
    ExitCode::SUCCESS
}
