//! The brep-bench runner for SolidCraft (https://solidcraft.io), on its B-rep kernel boundary,
//! the `solidcraft-kernel` crate.
//!
//!     brep-bench-solidcraft CASE OUT.stl
//!     brep-bench-solidcraft --version
//!
//! Builds the case `CASE` (`boolean/napkin-ring`) with its implementation,
//! `src/cases/boolean/napkin_ring.rs`, writes the solid's triangles to `OUT.stl` and prints one
//! line of JSON: the solid's faces by surface kind and whether it passes SolidCraft's own
//! validity check, or a refusal.
//!
//! SolidCraft measures a solid from its tessellations (`solidcraft_kernel::measure`: fine and
//! medium meshes, face by face), so the runner answers `"measured": "mesh"` and the harness takes
//! the volume and area from the triangles it writes: SolidCraft's own tessellation of the solid.
//! A kernel error is a refusal: the kernel declined, saying why (it turns a panic in the truck
//! crates under it into an error of its own, `KernelError::Internal`). A case may also refuse in
//! its own words, where the kernel has no operation for what it describes.
use serde_json::{Value, json};
use solidcraft_kernel::{Body, FaceSurface, KernelError, face_surface, measure};
use std::collections::BTreeMap;
use std::process::ExitCode;

mod solids;

/// A case's implementation: the solid it builds, or why it was not built.
type Build = fn() -> Result<Body, Refusal>;

/// Why a case was not built: a kernel error, in the kernel's own words, or a case that refuses
/// in its own because the kernel has no operation for what it describes.
#[derive(Debug)]
pub struct Refusal(String);

impl From<KernelError> for Refusal {
    fn from(e: KernelError) -> Self {
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

fn kind(s: Option<FaceSurface>) -> &'static str {
    match s {
        Some(FaceSurface::Plane { .. }) => "plane",
        Some(FaceSurface::Cylinder { .. }) => "cylinder",
        Some(FaceSurface::Cone { .. }) => "cone",
        Some(FaceSurface::Sphere { .. }) => "sphere",
        Some(FaceSurface::Torus { .. }) => "torus",
        // the kernel names only the analytic kinds; the rest are truck's B-spline, NURBS and
        // swept surfaces
        None => "other",
    }
}

/// A binary STL of the mesh's triangles. Each vertex is written from the mesh's one position
/// for it, so faces that share an edge share its points exactly, as the harness joins them.
fn stl(mesh: &solidcraft_geom::Mesh) -> Vec<u8> {
    let mut out = vec![0u8; 80];
    out[..10].copy_from_slice(b"solidcraft");
    out.extend((mesh.triangles.len() as u32).to_le_bytes());
    for t in &mesh.triangles {
        let [a, b, c] = mesh.tri(t).expect("the mesh's triangles index its positions");
        let n = (b - a).cross(c - a);
        let n = if n.len() > 0.0 { n * (1.0 / n.len()) } else { n };
        for v in [n, a, b, c] {
            for x in [v.x, v.y, v.z] {
                out.extend((x as f32).to_le_bytes());
            }
        }
        out.extend([0u8; 2]);
    }
    out
}

/// The faces and validity of `body`, its triangles written to `out`.
fn report(body: &Body, out: &str) -> Result<Value, KernelError> {
    let mut faces: BTreeMap<&str, usize> = BTreeMap::new();
    for f in 0..body.face_count() {
        *faces.entry(kind(face_surface(body, f))).or_default() += 1;
    }
    let problems = body.validity();
    // the chord of the written mesh: as fine as the kernel's own measure goes (5e-5 of the body's
    // size), and finer for a thin body (a fraction of volume over area, half a tube's radius), so
    // a thin pipe's volume and area are as close as a block's, well inside the harness' 1e-3
    let m = measure(body)?;
    let mut tol = body.size() * 5e-5;
    if m.area > 0.0 && m.volume > 0.0 {
        tol = tol.min(5e-4 * m.volume / m.area);
    }
    let mesh = body.display_mesh(tol.max(1e-5))?;
    std::fs::write(out, stl(&mesh)).expect("the STL file writes");
    let mut reply = json!({"status": "ok", "measured": "mesh", "faces": faces,
                           "valid": problems.is_empty()});
    if !problems.is_empty() {
        reply["problems"] = json!(problems);
    }
    Ok(reply)
}

fn refused(why: impl std::fmt::Display) -> Value {
    json!({"status": "refused", "reason": why.to_string()})
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().map(String::as_str) == Some("--version") {
        println!(
            "solidcraft-kernel {} (runner {})",
            env!("SOLIDCRAFT_VERSION"),
            env!("CARGO_PKG_VERSION")
        );
        return ExitCode::SUCCESS;
    }
    let [case, out] = &args[..] else {
        eprintln!("usage: brep-bench-solidcraft CASE OUT.stl | --version");
        return ExitCode::from(2);
    };
    let Some(&(_, build)) = cases::CASES.iter().find(|(id, _)| id == case) else {
        eprintln!("no implementation of `{case}`");
        return ExitCode::from(2);
    };
    let reply = match build().and_then(|body| Ok(report(&body, out)?)) {
        Ok(reply) => reply,
        Err(e) => refused(e),
    };
    println!("{reply}");
    ExitCode::SUCCESS
}
