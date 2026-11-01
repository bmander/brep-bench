//! The brep-bench runner for next.BREP's kernel
//! (https://github.com/mmiscool/next.BREP.io_RUST_BREP_KERNEL, `BREP_kernel` on crates.io).
//!
//!     brep-bench-next-brep CASE OUT.stl
//!     brep-bench-next-brep --version
//!
//! Builds the case `CASE` (`boolean/napkin-ring`) with its implementation,
//! `src/cases/boolean/napkin_ring.rs`, writes the solid's triangles to `OUT.stl` and prints one
//! line of JSON: the kernel's own measures of the solid (`solid_mass_properties`, integrated from
//! the B-rep), its faces by surface kind and whether it passes the kernel's validation, or a
//! refusal. A kernel error is a refusal: the kernel declined, saying why. A case may also refuse in
//! its own words, where the kernel has no operation for what it describes.
use brep_kernel::{
    solid_mass_properties, tessellate_brep_watertight, write_binary_stl, AnalyticSurface,
    BrepSolid, KernelRefusal,
};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::process::ExitCode;

#[allow(dead_code)]
mod solids;

/// A case's implementation: the solid it builds, or why it was not built.
type Build = fn() -> Result<BrepSolid, Refusal>;

/// Why a case was not built: the kernel's refusal, in its own words, or a case that refuses in its
/// own because the kernel has no operation for what it describes.
#[derive(Debug)]
pub struct Refusal(String);

impl From<KernelRefusal> for Refusal {
    fn from(e: KernelRefusal) -> Self {
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

/// The kind of a face's surface. Every surface is a NURBS surface; the kernel recognises the
/// analytic carrier it is exactly, where it is one.
fn kind(surface: &brep_kernel::NurbsSurface) -> &'static str {
    match surface.analytic() {
        Some(AnalyticSurface::Plane { .. }) => "plane",
        Some(AnalyticSurface::RuledRevolution { rho0, rho1, .. }) => {
            if (rho0 - rho1).abs() <= 1e-9 * rho0.abs().max(rho1.abs()).max(1.0) {
                "cylinder"
            } else {
                "cone"
            }
        }
        Some(AnalyticSurface::Sphere { .. }) => "sphere",
        Some(AnalyticSurface::Torus { .. }) => "torus",
        Some(AnalyticSurface::Revolution { .. }) => "other",
        None => "bspline",
    }
}

/// The measures, face kinds and validity of `solid`, its triangles written to `out`.
fn report(solid: &BrepSolid, out: &str) -> Result<Value, Refusal> {
    let props = solid_mass_properties(solid).map_err(|e| format!("not measured: {e}"))?;
    let mut faces: BTreeMap<&str, usize> = BTreeMap::new();
    for face in solid.shells.iter().flat_map(|s| &s.faces) {
        *faces.entry(kind(&face.surface)).or_default() += 1;
    }
    let valid = solid.validate().iter().all(|issue| issue.severity != "error");
    // a chord of a fraction of the body's thickness (volume over area: half a tube's radius), so a
    // thin pipe is meshed as closely as a block, well inside the harness' 2e-2 on the volume
    let chord = 5e-3 * props.volume.abs() / props.surface_area.max(1e-12);
    let mesh = tessellate_brep_watertight(solid, chord).map_err(|e| format!("not meshed: {e}"))?;
    let bytes = write_binary_stl(&mesh, "next.BREP").map_err(|e| format!("no STL: {e}"))?;
    std::fs::write(out, bytes).expect("the STL file writes");
    Ok(json!({
        "status": "ok", "volume": props.volume, "area": props.surface_area, "faces": faces,
        "valid": valid,
    }))
}

fn refused(why: impl std::fmt::Display) -> Value {
    json!({"status": "refused", "reason": why.to_string()})
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().map(String::as_str) == Some("--version") {
        println!("next.BREP BREP_kernel {} (runner {})", env!("KERNEL_VERSION"),
                 env!("CARGO_PKG_VERSION"));
        return ExitCode::SUCCESS;
    }
    let [case, out] = &args[..] else {
        eprintln!("usage: brep-bench-next-brep CASE OUT.stl | --version");
        return ExitCode::from(2);
    };
    let Some(&(_, build)) = cases::CASES.iter().find(|(id, _)| id == case) else {
        eprintln!("no implementation of `{case}`");
        return ExitCode::from(2);
    };
    let reply = match build().and_then(|solid| report(&solid, out)) {
        Ok(reply) => reply,
        Err(e) => refused(e),
    };
    println!("{reply}");
    ExitCode::SUCCESS
}
