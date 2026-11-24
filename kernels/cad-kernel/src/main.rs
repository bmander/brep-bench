//! The brep-bench runner for cad-kernel (https://crates.io/crates/cad-kernel).
//!
//!     brep-bench-cad-kernel CASE OUT.stl [OUT.step]      (CASE as `boolean/napkin-ring`)
//!     brep-bench-cad-kernel --version
//!
//! Builds the case with its own file, `src/cases/<category>/<name_snake>.rs`, then measures the
//! solid with the kernel (exact volume and area, faces by surface, its validity check), meshes it
//! into OUT.stl, writes it as STEP into OUT.step if asked (the kernel's own writer, `step::write`)
//! and prints one line of JSON. Given OUT.step, the STEP is the whole answer: it is written, and
//! nothing is measured or meshed unless the writer declines. A case that refuses, or a kernel
//! operation that declines, is a refusal saying why. The runner never reads the case's description.
use cad_kernel::geom::{tolerance, Surface};
use cad_kernel::mesh::mesh;
use cad_kernel::props::{area, volume};
use cad_kernel::topo::Brep;
use cad_kernel::util::json::Json;
use cad_kernel::util::space::{cross, sub, unit};
use std::collections::BTreeMap;

mod sketch;
mod tumble;
mod gears;
mod cnc;
mod patches;

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

/// The solid meshed to a sag of a ten-thousandth of its size, or of 1/200 of its thickness
/// (its volume over its area) where that is less — the reference runner's rule
/// (`kernels/occt/run.py`) — written as binary STL. A sag as deep as a thin solid is thick (a lens
/// a millionth thick) lets its two sides be meshed as one flat disc.
fn stl(b: &Brep, path: &str, volume: f64, area: f64) -> Result<(), String> {
    let thickness = if area > 0. && volume > 0. { volume / area } else { b.size() };
    let m = mesh(b, (1e-4 * b.size()).min(5e-3 * thickness), 0.1)?;
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

/// The solid written as STEP into `out_step`, if asked for: where the writer declines, the
/// reply's `step_error` field saying why (else nothing).
fn step(b: &Brep, out_step: Option<&str>, case: &str) -> String {
    let step_error = out_step.and_then(|path| {
        // no tolerance is asked of a case, so the file's fitted curves are held to the exact ones:
        // the harness measures the solid the kernel built
        cad_kernel::step::write(b, case, tolerance(b.size()), cad_kernel::step::Fit::Exact)
            .and_then(|text| std::fs::write(path, text).map_err(|e| e.to_string()))
            .err()
    });
    step_error
        .map(|e| format!(",\"step_error\":{}", Json::Str(e).dump(None)))
        .unwrap_or_default()
}

/// The reply for a solid built: its measures, once its triangles are written (and its STEP, if
/// asked for). A STEP written is the whole answer: nothing is then measured or meshed.
fn measured(b: &Brep, out: &str, out_step: Option<&str>, case: &str) -> String {
    // given OUT.step, the solid written there is the whole answer (SPEC.md, "A runner"): nothing
    // measured or meshed; only where the writer declines does the runner go on to its triangles
    let step_error = step(b, out_step, case);
    if out_step.is_some() && step_error.is_empty() {
        return "{\"status\":\"ok\"}".into();
    }
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
            "{{\"status\":\"ok\",\"volume\":{v:e},\"area\":{a:e},\"faces\":{{{faces}}},\"valid\":{valid}{step_error}}}"
        ),
        Err(e) => refused(format!("not meshed: {e}")),
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

/// The runner's own clock, for the kernel's own time (SPEC.md, "A runner"): started as the
/// runner's first pause ends, and stopped once the kernel has answered, its solid built and
/// written or its refusal in hand. The harness times the same between the two pauses, but a
/// process stopped and continued costs microseconds that are not the kernel's.
struct Clock {
    since: std::time::Instant,
    stopped: std::cell::Cell<Option<f64>>,
}

impl Clock {
    fn start() -> Clock {
        Clock { since: std::time::Instant::now(), stopped: std::cell::Cell::new(None) }
    }

    /// Stops the clock, unless it is stopped: its seconds.
    fn stop(&self) -> f64 {
        let seconds = self.stopped.get().unwrap_or_else(|| self.since.elapsed().as_secs_f64());
        self.stopped.set(Some(seconds));
        seconds
    }
}

/// The reply with the kernel's own time added to it, `seconds`, the clock stopped now if the
/// kernel's refusing did not stop it.
fn timed(reply: &impl std::fmt::Display, clock: &Clock) -> String {
    let seconds = clock.stop();
    let reply = reply.to_string();
    match reply.strip_suffix('}') {
        Some(open) => format!("{open},\"seconds\":{seconds:e}}}"),
        None => reply,
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some("--version") {
        println!("cad-kernel {} (brep-bench runner {})", env!("CAD_KERNEL_REV"), env!("CARGO_PKG_VERSION"));
        return;
    }
    let (case, out, out_step) = match &args[..] {
        [_, case, out] => (case, out, None),
        [_, case, out, step] => (case, out, Some(step.as_str())),
        _ => {
            eprintln!("usage: brep-bench-cad-kernel CASE OUT.stl [OUT.step]");
            std::process::exit(2);
        }
    };
    let Some(&(_, build)) = cases::CASES.iter().find(|(id, _)| id == case) else {
        eprintln!("no implementation of `{case}`");
        std::process::exit(2);
    };
    pause();
    let clock = Clock::start();
    let reply = match build() {
        Ok(solid) => measured(&solid, out, out_step, case),
        Err(why) => {
            clock.stop();
            refused(why)
        }
    };
    println!("{}", timed(&reply, &clock));
    pause();
}
