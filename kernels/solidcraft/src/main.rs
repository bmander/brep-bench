//! The brep-bench runner for SolidCraft (https://solidcraft.io), on its B-rep kernel boundary,
//! the `solidcraft-kernel` crate, and on the modelling layer under it, solidcore-ops (`ops`).
//!
//!     brep-bench-solidcraft CASE OUT.stl [OUT.step]
//!     brep-bench-solidcraft --version
//!
//! Builds the case `CASE` (`boolean/napkin-ring`) with its implementation,
//! `src/cases/boolean/napkin_ring.rs`, writes the solid's triangles to `OUT.stl` and prints one
//! line of JSON: the solid's volume and area, its faces by surface kind and whether it passes
//! SolidCraft's own validity check, or a refusal.
//!
//! Given `OUT.step`, it also writes the solid there as STEP (AP242) with the kernel's own writer,
//! `solidcraft_kernel::step_export`, the one SolidCraft's app exports a body with. Where the
//! writer declines the solid, no file is written and the reply says why in `step_error`. A STEP
//! written is the whole answer: the solid is then not meshed, and `OUT.stl` not written.
//!
//! The kernel under SolidCraft is solidcore, its Rust port of NVIDIA's SMLib, and
//! `solidcraft_kernel::measure` integrates a solid's volume and area on its exact surfaces
//! (SMLib's, at a relative accuracy of 1e-8; a mesh body's from its triangles, which the reply
//! then says, `"measured": "mesh"`), so those are the measures the runner answers; its
//! triangles are SolidCraft's own mesh of the solid (`Body::mesh`). A kernel error is a refusal:
//! the kernel declined, saying why (it turns a panic in the kernel under it into an error of its
//! own, `KernelError::Internal`). A case may also refuse in its own words, where the kernel has no
//! operation for what it describes. A boolean the kernel answers is empty is not a refusal but an
//! answer: the runner reports an empty solid.
//!
//! A case's `build` returns either kernel's body: a `solidcraft_kernel::Body`, or, for a case
//! built on solidcore-ops, a `solidcore_ops::Body` (`ops`), which is written as STEP by the same
//! writer and, where it declines, measured and meshed by solidcore's own tools. Which one is a
//! case's own choice, made in its file; the runner learns it from the type ([`Layer`]).
use serde_json::{Value, json};
use solidcraft_geom::prelude::*;
use solidcraft_kernel::{
    Body, FaceSurface, KernelError, MeshTolerance, face_surface, measure, step_export,
};
use std::collections::BTreeMap;
use std::process::ExitCode;

#[allow(dead_code)]
mod bench;
mod ops;
#[allow(dead_code, unused_imports)]
mod shared;
mod solids;

/// The solid a case built, on either layer.
pub enum Solid {
    /// Built through SolidCraft's kernel crate.
    Craft(Body),
    /// Built through solidcore-ops, beneath it.
    Core(solidcore_ops::Body),
}

/// A body type a case's `build` may return: what the runner sets up before building on its
/// layer, and the body as a [`Solid`].
pub trait Layer: Sized {
    fn prepare() -> Result<(), Refusal> {
        Ok(())
    }
    fn solid(self) -> Solid;
}

impl Layer for Body {
    fn solid(self) -> Solid {
        Solid::Craft(self)
    }
}

impl Layer for solidcore_ops::Body {
    fn prepare() -> Result<(), Refusal> {
        ops::set_fixes()
    }
    fn solid(self) -> Solid {
        Solid::Core(self)
    }
}

/// A case's `build` run on its layer: the layer set up, then the solid built.
pub fn run<B: Layer, E: Into<Refusal>>(build: fn() -> Result<B, E>) -> Result<Solid, Refusal> {
    B::prepare()?;
    build().map(B::solid).map_err(Into::into)
}

/// A case's implementation: the solid it builds, or why it was not built.
type Build = fn() -> Result<Solid, Refusal>;

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
    use super::{Build, run};
    include!(concat!(env!("OUT_DIR"), "/cases.rs"));
}

fn kind(s: Option<FaceSurface>) -> &'static str {
    match s {
        Some(FaceSurface::Plane { .. }) => "plane",
        Some(FaceSurface::Cylinder { .. }) => "cylinder",
        Some(FaceSurface::Cone { .. }) => "cone",
        Some(FaceSurface::Sphere { .. }) => "sphere",
        Some(FaceSurface::Torus { .. }) => "torus",
        // the kernel names only the analytic kinds; the rest are B-spline, NURBS and swept
        // surfaces
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
    let m = measure(body)?;
    // the chord of the written mesh: 5e-5 of the body's size, and finer for a thin body (a
    // fraction of volume over area, half a tube's radius), so that the harness's checks of the
    // triangles (closed, the expected volume within 2%) see a thin pipe as well as a block
    let mut tol = body.size() * 5e-5;
    if m.area > 0.0 && m.volume > 0.0 {
        tol = tol.min(5e-4 * m.volume / m.area);
    }
    let mesh = body.mesh(&MeshTolerance::chord(tol.max(1e-5)))?;
    std::fs::write(out, stl(&mesh)).expect("the STL file writes");
    let mut reply = json!({"status": "ok", "volume": m.volume, "area": m.area, "faces": faces,
                           "valid": problems.is_empty()});
    // a mesh body is measured from its own triangles, so its measures are the mesh's
    if m.measured_by == "mesh" {
        reply["measured"] = json!("mesh");
    }
    if !problems.is_empty() {
        reply["problems"] = json!(problems);
    }
    Ok(reply)
}

/// `bodies` written as STEP to `path` by SolidCraft's writer, or, where it declines them, why
/// (and no file).
fn step(bodies: &[&Body], path: &str) -> Option<String> {
    match step_export(bodies, "SolidCraft") {
        Ok(text) => {
            std::fs::write(path, text).expect("the STEP file writes");
            None
        }
        Err(e) => Some(e.to_string()),
    }
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
    let (case, out, out_step) = match &args[..] {
        [case, out] => (case, out, None),
        [case, out, step] => (case, out, Some(step.as_str())),
        _ => {
            eprintln!("usage: brep-bench-solidcraft CASE OUT.stl [OUT.step] | --version");
            return ExitCode::from(2);
        }
    };
    let Some(&(_, build)) = cases::CASES.iter().find(|(id, _)| id == case) else {
        eprintln!("no implementation of `{case}`");
        return ExitCode::from(2);
    };
    pause();
    let clock = Clock::start();
    let reply = match build().and_then(|solid| {
        let body = match solid {
            Solid::Craft(body) => body,
            Solid::Core(body) => {
                let declined = match out_step {
                    Some(path) => match ops::step(&body, path) {
                        None => return Ok(json!({"status": "ok"})),
                        why => why,
                    },
                    None => None,
                };
                let mut reply = ops::report(&body, out)?;
                if let Some(why) = declined {
                    reply["step_error"] = json!(why);
                }
                return Ok(reply);
            }
        };
        // given OUT.step, the solid written there is the whole answer (SPEC.md, "A runner"):
        // nothing meshed; only where the writer declines does the runner go on to its triangles
        let declined = match out_step {
            Some(path) => match step(&[&body], path) {
                None => return Ok(json!({"status": "ok"})),
                why => why,
            },
            None => None,
        };
        let mut reply = report(&body, out)?;
        if let Some(why) = declined {
            reply["step_error"] = json!(why);
        }
        Ok(reply)
    }) {
        Ok(reply) => reply,
        // the kernel said the solid is empty: no triangles, and the measures of nothing
        Err(Refusal(why)) if why.contains(solids::EMPTY) => {
            std::fs::write(out, stl(&solidcraft_geom::Mesh::default())).expect("the STL file writes");
            let mut reply = json!({"status": "ok", "volume": 0.0, "area": 0.0, "faces": {},
                                   "valid": false, "problems": [why]});
            // and no solid to write as STEP: no file, and the reply says so, in the runner's
            // words (the writer is not asked to write nothing)
            if out_step.is_some() {
                reply["step_error"] = json!("no solid to write: the kernel's answer is empty");
            }
            reply
        }
        Err(e) => {
            clock.stop();
            refused(e)
        }
    };
    println!("{}", timed(&reply, &clock));
    pause();
    ExitCode::SUCCESS
}
