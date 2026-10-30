//! The brep-bench runner for arris (https://github.com/arris-labs/arris).
//!
//!     brep-bench-arris CASE OUT.stl
//!     brep-bench-arris --version
//!
//! Builds the case `CASE` (`boolean/napkin-ring`) with its implementation,
//! `src/cases/boolean/napkin_ring.rs`, writes the solid's triangles to `OUT.stl` and prints one
//! line of JSON: arris's own measures of the solid, its faces by surface kind and whether it passes
//! arris's checker, or a refusal. An arris error is a refusal: the kernel declined, saying why. A
//! case may also refuse in its own words, where arris has no operation for what it describes.
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

/// The measures, face kinds and validity of `body`, its triangles written to `out`.
fn report(m: &Model, body: Body, out: &str) -> Result<Value, OpError> {
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
    Ok(json!({
        "status": "ok", "volume": props.volume, "area": props.area, "faces": faces, "valid": valid,
    }))
}

fn refused(why: impl std::fmt::Display) -> Value {
    json!({"status": "refused", "reason": why.to_string()})
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().map(String::as_str) == Some("--version") {
        println!("arris {} (runner {})", env!("ARRIS_VERSION"), env!("CARGO_PKG_VERSION"));
        return ExitCode::SUCCESS;
    }
    let [case, out] = &args[..] else {
        eprintln!("usage: brep-bench-arris CASE OUT.stl | --version");
        return ExitCode::from(2);
    };
    let Some(&(_, build)) = cases::CASES.iter().find(|(id, _)| id == case) else {
        eprintln!("no implementation of `{case}`");
        return ExitCode::from(2);
    };
    let mut m = Model::default();
    let reply = match build(&mut m).and_then(|body| Ok(report(&m, body, out)?)) {
        Ok(reply) => reply,
        Err(e) => refused(e),
    };
    println!("{reply}");
    ExitCode::SUCCESS
}
