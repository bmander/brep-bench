//! The brep-bench runner for AnvilCAD's exact kernel, anvil-brep
//! (https://github.com/swperb/anvilcad, `crates/anvil-brep`).
//!
//!     brep-bench-anvilcad CASE OUT.stl [OUT.step]      (CASE as `boolean/napkin-ring`)
//!     brep-bench-anvilcad --version
//!
//! Builds the case with its own file, `src/cases/<category>/<name_snake>.rs`, and, given OUT.step,
//! writes the solid there with AnvilCAD's own STEP writer (`anvil_step::export::write`, exact
//! AP214): that file is the whole answer, and nothing is measured or meshed.
//!
//! anvil-brep has no exact measures of its own: its mass properties are those of its
//! tessellation (`anvil_brep::mass`). So where there is no STEP to give, a built case answers
//! `"measured": "mesh"` and the harness takes the volume and area from the triangles, the kernel's
//! own tessellation of the solid (`anvil_brep::tess`), fine enough for the 1e-3 it asks. Every
//! solid anvil-brep returns has passed its own `check::validate`; a kernel error is a refusal in
//! the kernel's words, and a case may refuse in its own where the kernel has no operation for what
//! it describes. The runner never reads the case's description.
use anvil_brep::tess::{TessParams, tessellate};
use anvil_brep::{Solid, Surface, check};
use std::collections::BTreeMap;

mod shared;

/// What a case builds: one solid, or several where its description asks for separate bodies.
pub trait Bodies {
    fn bodies(self) -> Result<Vec<Solid>, String>;
}

impl Bodies for Result<Solid, String> {
    fn bodies(self) -> Result<Vec<Solid>, String> {
        self.map(|s| vec![s])
    }
}

impl Bodies for Result<Vec<Solid>, String> {
    fn bodies(self) -> Result<Vec<Solid>, String> {
        self
    }
}

/// Every case file, found by `build.rs`.
mod cases {
    include!(concat!(env!("OUT_DIR"), "/cases.rs"));
}

/// A face's surface as the reply counts it.
fn kind(s: &Surface) -> &'static str {
    match s {
        Surface::Plane { .. } => "plane",
        Surface::Cylinder { .. } => "cylinder",
        Surface::Cone { .. } => "cone",
        Surface::Sphere { .. } => "sphere",
        Surface::Torus { .. } => "torus",
        Surface::BSpline(..) => "bspline",
    }
}

/// A string as a JSON string.
fn json(s: &str) -> String {
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
    format!("{{\"status\":\"refused\",\"reason\":{}}}", json(why))
}

/// The solid tessellated with a chord of a ten-thousandth of its size, written as binary STL.
/// The error of a curved face's volume and area falls as the square of the chord, so this keeps a
/// ball within about 1e-5 of both, well inside the 1e-3 the harness asks of a mesh.
fn stl(solids: &[Solid], path: &str) -> Result<(), String> {
    let (mut lo, mut hi) = (glam::DVec3::INFINITY, glam::DVec3::NEG_INFINITY);
    for (a, b) in solids.iter().filter_map(Solid::bounds) {
        (lo, hi) = (lo.min(a), hi.max(b));
    }
    let size = (hi - lo).length();
    if solids.is_empty() {
        // nothing to tessellate: an STL of no triangles
        return std::fs::write(path, [0u8; 84]).map_err(|e| e.to_string());
    }
    if !size.is_finite() {
        return Err("the solid has no extent".into());
    }
    let params = TessParams {
        chord: 1e-4 * size,
        max_angle: std::f64::consts::PI / 180.0,
    };
    let mut triangles = Vec::new();
    for solid in solids {
        let m = tessellate(solid, &params).map_err(|e| e.to_string())?;
        triangles.extend(
            m.triangles
                .iter()
                .map(|t| t.map(|i| m.positions[i as usize])),
        );
    }
    let mut out = Vec::with_capacity(84 + 50 * triangles.len());
    out.extend_from_slice(&[0u8; 80]);
    out.extend_from_slice(&(triangles.len() as u32).to_le_bytes());
    for t in &triangles {
        let [a, b, c] = *t;
        let n = (b - a).cross(c - a).normalize_or_zero();
        for p in [n, a, b, c] {
            for x in p.to_array() {
                out.extend_from_slice(&(x as f32).to_le_bytes());
            }
        }
        out.extend_from_slice(&[0u8; 2]);
    }
    std::fs::write(path, out).map_err(|e| e.to_string())
}

/// The reply for a solid built. Given OUT.step the solid is written there as STEP, and that is
/// the whole answer; otherwise its faces by surface, the kernel's validity check, and its
/// triangles, from which the harness measures it.
fn answer(solids: &[Solid], out: &str, out_step: Option<&str>, case: &str) -> String {
    // anvil-step's writer has no way to decline a solid: what it writes, or the file not
    // written, is all there is to say
    let step_error = out_step.and_then(|path| {
        std::fs::write(path, anvil_step::export::write(solids, case))
            .err()
            .map(|e| e.to_string())
    });
    if out_step.is_some() && step_error.is_none() {
        return "{\"status\":\"ok\"}".into();
    }
    let step_error = step_error
        .map(|e| format!(",\"step_error\":{}", json(&e)))
        .unwrap_or_default();
    let mut faces = BTreeMap::new();
    for f in solids.iter().flat_map(|s| &s.faces) {
        *faces.entry(kind(&f.surface)).or_insert(0) += 1;
    }
    let faces = faces
        .iter()
        .map(|(k, n)| format!("\"{k}\":{n}"))
        .collect::<Vec<_>>()
        .join(",");
    let valid = solids.iter().all(|s| check::validate(s).is_ok());
    match stl(solids, out) {
        Ok(()) => format!(
            "{{\"status\":\"ok\",\"measured\":\"mesh\",\"faces\":{{{faces}}},\"valid\":{valid}{step_error}}}"
        ),
        Err(e) => refused(&format!("not tessellated: {e}")),
    }
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

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some("--version") {
        println!(
            "anvil-brep {} (brep-bench runner {})",
            env!("ANVIL_BREP_REV"),
            env!("CARGO_PKG_VERSION")
        );
        return;
    }
    let (case, out, out_step) = match &args[..] {
        [_, case, out] => (case, out, None),
        [_, case, out, step] => (case, out, Some(step.as_str())),
        _ => {
            eprintln!("usage: brep-bench-anvilcad CASE OUT.stl [OUT.step]");
            std::process::exit(2);
        }
    };
    let Some(&(_, build)) = cases::CASES.iter().find(|(id, _)| id == case) else {
        eprintln!("no implementation of `{case}`");
        std::process::exit(2);
    };
    pause();
    let reply = match build() {
        Ok(solids) => answer(&solids, out, out_step, case),
        // the kernel's operation succeeded and left nothing: that is its answer, written as such
        Err(why) if why == shared::NOTHING => answer(&[], out, out_step, case),
        Err(why) => refused(&why),
    };
    println!("{reply}");
    pause();
}
