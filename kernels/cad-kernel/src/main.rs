//! The brep-bench runner for cad-kernel (https://crates.io/crates/cad-kernel).
//!
//!     brep-bench-cad-kernel CASE OUT.stl      (CASE as `boolean/napkin-ring`)
//!     brep-bench-cad-kernel --version
//!
//! Builds the case with its own file, `src/cases/<category>/<name_snake>.rs`, then measures the
//! solid with the kernel (exact volume and area, faces by surface, its validity check), meshes it
//! into OUT.stl and prints one line of JSON. A case that refuses, or a kernel operation that
//! declines, is a refusal saying why. The runner never reads the case's description.
use cad_kernel::geom::{tolerance, Surface};
use cad_kernel::mesh::mesh;
use cad_kernel::props::{area, volume};
use cad_kernel::topo::Brep;
use cad_kernel::util::json::Json;
use cad_kernel::util::space::{cross, sub, unit};
use std::collections::BTreeMap;

mod sketch;

/// Every case file, found by `build.rs`.
mod cases {
    include!(concat!(env!("OUT_DIR"), "/cases.rs"));
}

/// A face's surface as the reply counts it.
fn kind(s: &Surface) -> &'static str {
    match s {
        Surface::Plane(..) => "plane",
        Surface::Cylinder(..) => "cylinder",
        Surface::Cone(..) => "cone",
        Surface::Sphere(..) => "sphere",
        Surface::Torus(..) => "torus",
        Surface::BSpline(..) => "bspline",
        _ => "other",
    }
}

/// The solid meshed to a sag of a ten-thousandth of its size or of its thickness, whichever is
/// less, written as binary STL. Its thickness is three times its volume over its area (a ball's
/// radius, half a slab's depth): a sag as deep as a thin solid is thick (a lens a millionth
/// thick) lets its two sides be meshed as one flat disc.
fn stl(b: &Brep, path: &str, volume: f64, area: f64) -> Result<(), String> {
    let thickness = if area > 0. && volume > 0. { 3. * volume / area } else { b.size() };
    let m = mesh(b, 1e-4 * b.size().min(thickness), 0.1)?;
    let mut out = Vec::with_capacity(84 + 50 * m.tris.len());
    out.extend_from_slice(&[0u8; 80]);
    out.extend_from_slice(&(m.tris.len() as u32).to_le_bytes());
    for t in &m.tris {
        let [a, b, c] = t.map(|i| m.pts[i as usize]);
        let n = unit(cross(sub(b, a), sub(c, a)));
        for p in [n, a, b, c] {
            for x in p {
                out.extend_from_slice(&(x as f32).to_le_bytes());
            }
        }
        out.extend_from_slice(&[0u8; 2]);
    }
    std::fs::write(path, out).map_err(|e| e.to_string())
}

fn refused(why: String) -> String {
    format!("{{\"status\":\"refused\",\"reason\":{}}}", Json::Str(why).dump(None))
}

/// The reply for a solid built: its measures, once its triangles are written.
fn measured(b: &Brep, out: &str) -> String {
    let mut faces = BTreeMap::new();
    for f in &b.faces {
        *faces.entry(kind(&f.surface)).or_insert(0) += 1;
    }
    let faces = faces.iter().map(|(k, n)| format!("\"{k}\":{n}")).collect::<Vec<_>>().join(",");
    let a: f64 = b.faces.iter().map(|f| area(b, f)).sum();
    let valid = b.check(tolerance(b.size())).is_ok();
    let v = volume(b);
    match stl(b, out, v, a) {
        Ok(()) => format!(
            "{{\"status\":\"ok\",\"volume\":{v:e},\"area\":{a:e},\"faces\":{{{faces}}},\"valid\":{valid}}}"
        ),
        Err(e) => refused(format!("not meshed: {e}")),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some("--version") {
        println!("cad-kernel {} (brep-bench runner {})", env!("CAD_KERNEL_REV"), env!("CARGO_PKG_VERSION"));
        return;
    }
    let [_, case, out] = &args[..] else {
        eprintln!("usage: brep-bench-cad-kernel CASE OUT.stl");
        std::process::exit(2);
    };
    let Some(&(_, build)) = cases::CASES.iter().find(|(id, _)| id == case) else {
        eprintln!("no implementation of `{case}`");
        std::process::exit(2);
    };
    let reply = match build() {
        Ok(solid) => measured(&solid, out),
        Err(why) => refused(why),
    };
    println!("{reply}");
}
