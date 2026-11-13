//! The brep-bench runner for next.BREP's kernel
//! (https://github.com/mmiscool/next.BREP.io_RUST_BREP_KERNEL, `BREP_kernel` on crates.io).
//!
//!     brep-bench-next-brep CASE OUT.stl [OUT.step]
//!     brep-bench-next-brep --version
//!
//! Builds the case `CASE` (`boolean/napkin-ring`) with its implementation,
//! `src/cases/boolean/napkin_ring.rs`, writes the solid's triangles to `OUT.stl` (and, if asked,
//! the solid to `OUT.step` with the kernel's own STEP writer, `export_step`, AP242) and prints one
//! line of JSON: the kernel's own measures of the solid (`solid_mass_properties`, integrated from
//! the B-rep), its faces by surface kind and whether it passes the kernel's validation, or a
//! refusal. Given OUT.step, the STEP is the whole answer: it is written, and nothing is measured or
//! meshed unless the writer declines. A kernel error is a refusal: the kernel declined, saying why.
//! A case may also refuse in its own words, where the kernel has no operation for what it
//! describes.
use brep_kernel::{
    export_step, solid_mass_properties, tessellate_brep_watertight, write_binary_stl, AnalyticSurface,
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

/// The finest chord: a fraction of the body's thickness (volume over area: half a tube's radius),
/// so a thin pipe is meshed as closely as a block.
const FINEST: f64 = 5e-3;

/// The chords tried, coarser first, as multiples of the finest. A rung that fails costs a
/// tessellation, so there are two.
const LADDER: [f64; 2] = [16.0, 1.0];

/// How near the volume the triangles bound must be to the solid's for a chord coarser than the
/// finest to be kept: half what the harness asks (2e-2).
const MESH_VOLUME: f64 = 1e-2;

/// The STL of `solid`, and the chord it was meshed at: the coarsest of [`LADDER`] whose triangles,
/// as the STL holds them, are closed (joined where their points are equal, every edge used once
/// each way) and bound the solid's `volume` to [`MESH_VOLUME`], else the finest. A dense mesh of a
/// thin body is most of what such a case costs (the twisted thin blade: 590 000 triangles, 25 s, at
/// the finest). Binary STL holds single precision; where that joins points the mesher kept apart,
/// or moves the volume, the STL is ASCII with every coordinate in full.
fn triangles(solid: &BrepSolid, volume: f64, area: f64) -> Result<(Vec<u8>, f64), Refusal> {
    let finest = FINEST * volume.abs() / area.max(1e-12);
    for (rung, factor) in LADDER.iter().enumerate() {
        let chord = finest * factor;
        let last = rung + 1 == LADDER.len();
        let mesh = match tessellate_brep_watertight(solid, chord) {
            Ok(mesh) => mesh,
            Err(e) if last => return Err(format!("not meshed: {e}").into()),
            Err(_) => continue,
        };
        let points64: Vec<[f64; 3]> = mesh.positions.chunks_exact(3).map(|p| [p[0], p[1], p[2]]).collect();
        let points32: Vec<[f64; 3]> = points64.iter().map(|p| p.map(|x| x as f32 as f64)).collect();
        let exact = sound(&points64, &mesh.indices);
        let single = sound(&points32, &mesh.indices);
        let near = |bound: Option<f64>, rel: f64| bound.is_some_and(|v| (v - volume).abs() <= rel * volume.abs());
        // binary unless single precision opens what the mesher closed or moves its volume
        let binary = single.is_some_and(|v| exact.is_none_or(|w| (v - w).abs() <= 1e-4 * w.abs()))
            || exact.is_none();
        if !last && !near(if binary { single } else { exact }, MESH_VOLUME) {
            continue;
        }
        let bytes = if binary {
            write_binary_stl(&mesh, "next.BREP").map_err(|e| format!("no STL: {e}"))?
        } else {
            ascii_stl(&points64, &mesh.indices)
        };
        return Ok((bytes, chord));
    }
    unreachable!("the last rung returns")
}

/// The volume the triangles bound when they are closed as the harness reads them: points joined
/// where equal, a triangle with two equal corners left out, every directed edge used once and its
/// reverse once. None when they are not.
fn sound(points: &[[f64; 3]], indices: &[u32]) -> Option<f64> {
    use std::collections::HashMap;
    let mut ids: HashMap<[u64; 3], usize> = HashMap::with_capacity(points.len());
    let id: Vec<usize> = points
        .iter()
        .map(|p| {
            let next = ids.len();
            *ids.entry(p.map(f64::to_bits)).or_insert(next)
        })
        .collect();
    let mut uses: HashMap<(usize, usize), u32> = HashMap::with_capacity(indices.len());
    let mut volume = 0.0;
    for t in indices.chunks_exact(3) {
        let [a, b, c] = [t[0], t[1], t[2]].map(|i| i as usize);
        let [ia, ib, ic] = [id[a], id[b], id[c]];
        if ia == ib || ib == ic || ic == ia {
            continue;
        }
        for e in [(ia, ib), (ib, ic), (ic, ia)] {
            *uses.entry(e).or_default() += 1;
        }
        let [p, q, r] = [points[a], points[b], points[c]];
        volume += (p[0] * (q[1] * r[2] - q[2] * r[1]) - p[1] * (q[0] * r[2] - q[2] * r[0])
            + p[2] * (q[0] * r[1] - q[1] * r[0]))
            / 6.0;
    }
    let closed = !uses.is_empty() && uses.iter().all(|(&(i, j), &n)| n == 1 && uses.get(&(j, i)) == Some(&1));
    closed.then_some(volume)
}

/// ASCII STL, each coordinate the shortest decimal that reads back as the same double.
fn ascii_stl(points: &[[f64; 3]], indices: &[u32]) -> Vec<u8> {
    use std::fmt::Write;
    let mut text = String::from("solid next.BREP\n");
    for t in indices.chunks_exact(3) {
        let _ = writeln!(text, "facet normal 0 0 0\nouter loop");
        for &i in t {
            let [x, y, z] = points[i as usize];
            let _ = writeln!(text, "vertex {x:?} {y:?} {z:?}");
        }
        let _ = writeln!(text, "endloop\nendfacet");
    }
    text.push_str("endsolid next.BREP\n");
    text.into_bytes()
}

/// With `out_step`, `solid` written there as STEP; why the kernel's writer would not, if it would not.
fn step(solid: &BrepSolid, out_step: Option<&str>, case: &str) -> Option<String> {
    if let Some(path) = out_step {
        // the body is in millimetres, the unit the file is labelled with; no time stamp, so the
        // same case writes the same file
        match export_step(std::slice::from_ref(solid), case, "millimeter", "") {
            Ok(step) => std::fs::write(path, step).expect("the STEP file writes"),
            Err(e) => return Some(e),
        }
    }
    None
}

/// The measures, face kinds and validity of `solid`, its triangles written to `out`; with
/// `out_step`, the solid written there as STEP (`step`). A STEP written is the whole answer:
/// nothing is then measured or meshed.
fn report(solid: &BrepSolid, out: &str, out_step: Option<&str>, case: &str) -> Result<Value, Refusal> {
    // given OUT.step, the solid written there is the whole answer (SPEC.md, "A runner"): nothing
    // measured or meshed; only where the writer declines does the runner go on to its triangles
    let declined = step(solid, out_step, case);
    if out_step.is_some() && declined.is_none() {
        return Ok(json!({"status": "ok"}));
    }
    let props = solid_mass_properties(solid).map_err(|e| format!("not measured: {e}"))?;
    let mut faces: BTreeMap<&str, usize> = BTreeMap::new();
    for face in solid.shells.iter().flat_map(|s| &s.faces) {
        *faces.entry(kind(&face.surface)).or_default() += 1;
    }
    let valid = solid.validate().iter().all(|issue| issue.severity != "error");
    let (bytes, _) = triangles(solid, props.volume, props.surface_area)?;
    std::fs::write(out, bytes).expect("the STL file writes");
    let mut reply = json!({
        "status": "ok", "volume": props.volume, "area": props.surface_area, "faces": faces,
        "valid": valid,
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
        println!("next.BREP BREP_kernel {} (runner {})", env!("KERNEL_VERSION"),
                 env!("CARGO_PKG_VERSION"));
        return ExitCode::SUCCESS;
    }
    let (case, out, out_step) = match &args[..] {
        [case, out] => (case, out, None),
        [case, out, step] => (case, out, Some(step.as_str())),
        _ => {
            eprintln!("usage: brep-bench-next-brep CASE OUT.stl [OUT.step] | --version");
            return ExitCode::from(2);
        }
    };
    let Some(&(_, build)) = cases::CASES.iter().find(|(id, _)| id == case) else {
        eprintln!("no implementation of `{case}`");
        return ExitCode::from(2);
    };
    pause();
    let reply = match build().and_then(|solid| report(&solid, out, out_step, case)) {
        Ok(reply) => reply,
        Err(e) => refused(e),
    };
    println!("{reply}");
    pause();
    ExitCode::SUCCESS
}
