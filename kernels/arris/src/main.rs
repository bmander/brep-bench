//! The brep-bench runner for arris (https://github.com/arris-labs/arris).
//!
//!     brep-bench-arris CASE OUT.stl [OUT.step]
//!     brep-bench-arris --version
//!
//! Builds the case `CASE` (`boolean/napkin-ring`) with its implementation,
//! `src/cases/boolean/napkin_ring.rs`, writes the solid's triangles to `OUT.stl` (and, if asked,
//! the solid to `OUT.step` with arris's own STEP writer, `arris::io::step::write`, which writes
//! AP214) and prints one line of JSON: arris's own measures of the solid, its faces by surface kind
//! and whether it passes arris's checker, or a refusal. Given OUT.step, the STEP is the whole
//! answer: it is written, and nothing is measured or meshed unless the writer declines. An arris
//! error is a refusal: the kernel declined, saying why. A case may also refuse in its own words,
//! where arris has no operation for what it describes.
use arris::check::{check, Level};
use arris::geom::SurfaceKind;
use arris::math::Control;
use arris::ops::measure::mass_properties;
use arris::ops::OpError;
use arris::topo::{Body, Model};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::process::ExitCode;

mod solids;

/// A case's implementation: the solid it builds in `m`, or why it was not built.
type Build = fn(&mut Model) -> Result<Body, Refusal>;

/// Why a case was not built: an arris error, in arris's own words, or a case that refuses in its
/// own because arris has no operation for what it describes.
#[derive(Debug)]
pub struct Refusal(String);

impl From<OpError> for Refusal {
    fn from(e: OpError) -> Self {
        Refusal(e.to_string())
    }
}

impl From<arris::math::FrameError> for Refusal {
    fn from(e: arris::math::FrameError) -> Self {
        Refusal::from(OpError::from(e))
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

fn kind(k: SurfaceKind) -> &'static str {
    match k {
        SurfaceKind::Plane => "plane",
        SurfaceKind::Cylinder => "cylinder",
        SurfaceKind::Cone => "cone",
        SurfaceKind::Sphere => "sphere",
        SurfaceKind::Torus => "torus",
        SurfaceKind::Nurbs => "bspline",
        SurfaceKind::EllipticCylinder => "other",
    }
}

/// With `out_step`, `body` written there as STEP; why arris would not, if it would not.
fn step(m: &Model, body: Body, out_step: Option<&str>) -> Option<String> {
    if let Some(path) = out_step {
        match arris::io::step::write(m, &[body]) {
            Ok(text) => std::fs::write(path, text).expect("the STEP file writes"),
            Err(e) => return Some(e.to_string()),
        }
    }
    None
}

/// The measures, face kinds and validity of `body`, its triangles written to `out`; with
/// `out_step`, the body written there as STEP (`step`). A STEP written is the whole answer: nothing
/// is then measured or meshed.
fn report(m: &Model, body: Body, out: &str, out_step: Option<&str>) -> Result<Value, OpError> {
    // given OUT.step, the solid written there is the whole answer (SPEC.md, "A runner"): nothing
    // measured or meshed; only where the writer declines does the runner go on to its triangles
    let declined = step(m, body, out_step);
    if out_step.is_some() && declined.is_none() {
        return Ok(json!({"status": "ok"}));
    }
    let props = mass_properties(m, body, &Control::NONE)?;
    let mut faces: BTreeMap<&str, usize> = BTreeMap::new();
    for f in m.faces(body)? {
        let surface = m.surface(m.face(f.id)?.surface())?;
        *faces.entry(kind(surface.kind())).or_default() += 1;
    }
    // arris's fast check: `valid` is reported, not graded, and the full check's face-against-face
    // search can take minutes on a swept NURBS body the kernel built in milliseconds (a body the
    // checker rejects is still refused: mass_properties checks before it measures)
    let valid = check(m, body, Level::Fast).is_ok();
    // a chord of a fraction of the body's thickness (volume over area: half a tube's radius), so a
    // thin pipe is meshed as closely as a block, well inside the harness' 2e-2 on the volume
    let chord = 5e-3 * props.volume / props.area;
    let mesh = match arris::mesh::tessellate(m, body, chord, &Control::NONE) {
        Ok(mesh) => mesh,
        Err(e) => return Ok(refused(format!("not meshed: {e}"))),
    };
    let bytes = match arris::io::stl::write_binary(std::slice::from_ref(&mesh), "arris") {
        Ok(bytes) => bytes,
        Err(e) => return Ok(refused(format!("no STL written: {e}"))),
    };
    std::fs::write(out, bytes).expect("the STL file writes");
    let mut reply = json!({
        "status": "ok", "volume": props.volume, "area": props.area, "faces": faces, "valid": valid,
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
        println!("arris {} (runner {})", env!("ARRIS_VERSION"), env!("CARGO_PKG_VERSION"));
        return ExitCode::SUCCESS;
    }
    let (case, out, out_step) = match &args[..] {
        [case, out] => (case, out, None),
        [case, out, step] => (case, out, Some(step.as_str())),
        _ => {
            eprintln!("usage: brep-bench-arris CASE OUT.stl [OUT.step] | --version");
            return ExitCode::from(2);
        }
    };
    let Some(&(_, build)) = cases::CASES.iter().find(|(id, _)| id == case) else {
        eprintln!("no implementation of `{case}`");
        return ExitCode::from(2);
    };
    pause();
    let mut m = Model::default();
    let reply = match build(&mut m).and_then(|body| Ok(report(&m, body, out, out_step)?)) {
        Ok(reply) => reply,
        Err(e) => refused(e),
    };
    println!("{reply}");
    pause();
    ExitCode::SUCCESS
}
