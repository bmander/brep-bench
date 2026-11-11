//! The brep-bench runner for kentron (https://github.com/rsz269/kentron, `kentron` on crates.io).
//!
//!     brep-bench-kentron CASE OUT.stl [OUT.step]
//!     brep-bench-kentron --version
//!
//! Builds the case `CASE` (`boolean/napkin-ring`) with its implementation,
//! `src/cases/boolean/napkin_ring.rs`, in a fresh `Model`, then measures the solid with the
//! kernel's own tools: its exact volume and area (`properties`, integrated over the B-rep), its
//! faces by surface kind and its validity check (`validity::check`). It triangulates the solid
//! into OUT.stl, writes it as STEP into OUT.step if asked (the kernel's own writer,
//! `io::step::write`, AP214) and prints one line of JSON. Given `-` for OUT.stl (the harness's
//! timed run), it builds the solid and writes its STEP, and measures and meshes nothing. A kernel
//! error is a refusal: the kernel declined, saying why. A case may also refuse in its own words,
//! where kentron has no operation for what it describes. The runner never reads the case's
//! description.
use kentron::geometry::surface::Surface;
use kentron::math::Point;
use kentron::meshing::{triangulate, MeshOptions};
use kentron::properties;
use kentron::topology::{Model, NodeKind, Shape};
use kentron::validity;
use std::collections::BTreeMap;
use std::process::ExitCode;

#[allow(dead_code)]
mod bench;
#[allow(dead_code)]
mod shared;

/// A case's implementation: the solid it builds in the model, or why it was not built.
type Build = fn(&mut Model) -> Result<Shape, Refusal>;

/// Why a case was not built: kentron's error, in its own words, or a case that refuses in its
/// own because kentron has no operation for what it describes.
#[derive(Debug)]
pub struct Refusal(String);

impl From<kentron::Error> for Refusal {
    fn from(e: kentron::Error) -> Self {
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
        Surface::Plane(_) => "plane",
        Surface::Cylinder(_) => "cylinder",
        Surface::Cone(_) => "cone",
        Surface::Sphere(_) => "sphere",
        Surface::Torus(_) => "torus",
        Surface::BSpline(_) | Surface::Bezier(_) => "bspline",
        Surface::Trimmed(t) => kind(t.basis()),
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
fn write_stl(path: &str, nodes: &[Point], triangles: &[[u32; 3]]) -> std::io::Result<()> {
    let mut out = Vec::with_capacity(84 + 50 * triangles.len());
    out.extend_from_slice(&[0u8; 80]);
    out.extend_from_slice(&(triangles.len() as u32).to_le_bytes());
    for t in triangles {
        let [a, b, c] = t.map(|i| nodes[i as usize]);
        let n = (b - a).cross(c - a);
        let l = n.length();
        let n = if l > 0.0 { n * (1.0 / l) } else { n };
        for x in [n.x, n.y, n.z, a.x, a.y, a.z, b.x, b.y, b.z, c.x, c.y, c.z] {
            out.extend_from_slice(&(x as f32).to_le_bytes());
        }
        out.extend_from_slice(&[0u8; 2]);
    }
    std::fs::write(path, out)
}

/// The reply for a solid built: its measures, once its triangles are written. The mesh keeps
/// within a ten-thousandth of the solid's size, or 1/200 of its thickness (its volume over its
/// area) where that is less: the reference runner's rule (`kernels/occt/run.py`). With `out_step`,
/// the solid is also written there as STEP, named after the case; where the writer declines, no
/// file is written and the reply says why in `step_error`. Given `-` for `out`, the harness's
/// timed run, its STEP alone: nothing measured or meshed.
fn measured(m: &mut Model, solid: Shape, out: &str, out_step: Option<&str>, case: &str) -> String {
    if out == "-" {
        let step_error = out_step.and_then(|path| write_step(m, solid, case, path));
        return format!("{{\"status\":\"ok\"{}}}", step_error.unwrap_or_default());
    }
    let mut faces = BTreeMap::new();
    for f in m.explore(solid, NodeKind::Face).collect::<Vec<_>>() {
        let k = m.surface(f).map_or("other", kind);
        *faces.entry(k).or_insert(0) += 1;
    }
    let faces = faces.iter().map(|(k, n)| format!("\"{k}\":{n}")).collect::<Vec<_>>().join(",");
    let volume = properties::volume(m, solid).mass;
    let area = properties::area(m, solid).mass;
    let valid = validity::check(m, solid).is_valid();
    let size = properties::bounding_box(m, solid).size().length();
    let thickness = if area > 0.0 && volume > 0.0 { volume / area } else { size };
    let deflection = (1e-4 * size).min(5e-3 * thickness).max(1e-7);
    let options = MeshOptions { deflection, angle: 0.1, relative: false };
    // Meshes left on the solid by the building (a face picked by its distance to a point meshes
    // it coarsely) are dropped, so every face is meshed afresh to this one deflection: kentron
    // keeps a plane's old mesh as exact while it remeshes its curved edges finer.
    m.clear_meshes(solid);
    match triangulate(m, solid, options) {
        Ok((nodes, triangles)) => match write_stl(out, &nodes, &triangles) {
            Ok(()) => {
                let step_error = out_step.and_then(|path| write_step(m, solid, case, path)).unwrap_or_default();
                format!(
                    "{{\"status\":\"ok\",\"volume\":{volume:e},\"area\":{area:e},\"faces\":{{{faces}}},\"valid\":{valid}{step_error}}}"
                )
            }
            Err(e) => refused(&format!("{out} not written: {e}")),
        },
        Err(e) => refused(&format!("not meshed: {e}")),
    }
}

/// The solid written to `path` as STEP by kentron's writer, or, where it declines, the reply's
/// `step_error` field saying why in its own words. The file is written whole once the writer has
/// finished, so a refusal leaves none.
fn write_step(m: &Model, solid: Shape, case: &str, path: &str) -> Option<String> {
    let mut step = Vec::new();
    match kentron::io::step::write(m, solid, case, None, &mut step) {
        Ok(()) => {
            std::fs::write(path, step).unwrap_or_else(|e| panic!("{path} not written: {e}"));
            None
        }
        Err(e) => Some(format!(",\"step_error\":{}", quoted(&e.to_string()))),
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some("--version") {
        println!("kentron {} (brep-bench runner {})", env!("KERNEL_VERSION"), env!("CARGO_PKG_VERSION"));
        return ExitCode::SUCCESS;
    }
    let (case, out, out_step) = match &args[..] {
        [_, case, out] => (case, out, None),
        [_, case, out, step] => (case, out, Some(step.as_str())),
        _ => {
            eprintln!("usage: brep-bench-kentron CASE OUT.stl [OUT.step]");
            return ExitCode::from(2);
        }
    };
    let Some(&(_, build)) = cases::CASES.iter().find(|(id, _)| id == case) else {
        eprintln!("no implementation of `{case}`");
        return ExitCode::from(2);
    };
    let mut model = Model::new();
    let reply = match build(&mut model) {
        Ok(solid) => measured(&mut model, solid, out, out_step, case),
        Err(why) => refused(&why.to_string()),
    };
    println!("{reply}");
    ExitCode::SUCCESS
}
