//! The brep-bench runner for truck (https://github.com/ricosjp/truck).
//!
//!     brep-bench-truck CASE OUT.stl
//!     brep-bench-truck --version
//!
//! `CASE` is a case id such as `boolean/napkin-ring`. Each case is its own truck program, the
//! file `src/cases/<category>/<name_snake>.rs`, whose `build` function makes the solid its
//! description asks for (`build.rs` finds the files and writes the table `CASES` from them). The
//! runner never reads the case's description: it calls `build`, tessellates what comes back,
//! writes the triangles to `OUT.stl` and prints one line of JSON.
//!
//! truck measures only meshes, so a built case answers `"measured": "mesh"` and the harness takes
//! the volume and area from the triangles: truck's own tessellation of the solid, welded by
//! truck's own filter. A case truck cannot build answers `refused`, naming what truck lacks or
//! which truck operation declined. A panic inside truck is left to end the process.
use serde_json::json;
use std::collections::BTreeMap;
use truck_meshalgo::prelude::*;
use truck_modeling::{Solid, Surface};

mod rounded;
mod shapes;

include!(concat!(env!("OUT_DIR"), "/cases.rs"));

/// The tolerance of the written mesh, relative to the size of the solid (`shapes::size`).
/// truck's tessellation keeps each curve and surface within it, and the error of the volume and
/// area grows linearly with it: at this one a torus is within 5e-4 of both, half the 1e-3 the
/// harness asks.
const MESH_TOL: f64 = 4e-5;

/// What a case made: a solid of truck-modeling's, or one with an edge rounded, which carries the
/// fillet's own geometry types (see `rounded`).
pub enum Shape {
    Plain(Solid),
    Rounded(rounded::Solid),
}

impl From<Solid> for Shape {
    fn from(s: Solid) -> Shape { Shape::Plain(s) }
}

impl From<rounded::Solid> for Shape {
    fn from(s: rounded::Solid) -> Shape { Shape::Rounded(s) }
}

/// Why a case is not built: truck cannot, or one of its operations declined.
#[derive(Debug)]
pub struct Refused(pub String);

impl Refused {
    /// A truck operation that answered `None`, named as `truck-shapeops `and``.
    pub fn none_from(operation: &str) -> Refused {
        Refused(format!("{operation} returned None"))
    }
}

impl From<truck_modeling::errors::Error> for Refused {
    fn from(e: truck_modeling::errors::Error) -> Refused {
        Refused(format!("truck-modeling: {e}"))
    }
}

impl From<truck_topology::errors::Error> for Refused {
    fn from(e: truck_topology::errors::Error) -> Refused {
        Refused(format!("truck-topology: {e}"))
    }
}

/// What a case's `build` returns.
pub type Built = Result<Shape, Refused>;

fn kind(s: &Surface) -> &'static str {
    match s {
        Surface::Plane(_) => "plane",
        Surface::BSplineSurface(_) | Surface::NurbsSurface(_) => "bspline",
        Surface::RevolutedCurve(_) => "other",
    }
}

/// truck's tessellation of the solid, its faces' meshes joined where they share a vertex
/// (`put_together_same_attrs`, as truck's documentation closes a solid's mesh), as binary STL.
fn write_stl(mut mesh: PolygonMesh, path: &str) -> Result<(), Refused> {
    mesh.put_together_same_attrs(TOLERANCE)
        .remove_degenerate_faces()
        .remove_unused_attrs();
    let file = std::fs::File::create(path).map_err(|e| Refused(e.to_string()))?;
    let mut out = std::io::BufWriter::new(file);
    stl::write(&mesh, &mut out, stl::StlType::Binary)
        .map_err(|e| Refused(format!("the STL is not written: {e}")))
}

fn answer(shape: &Shape, out: &str) -> Result<serde_json::Value, Refused> {
    let mut faces = BTreeMap::new();
    // `valid` is truck-topology's check of a solid: each shell connected, closed and manifold
    let (mesh, valid) = match shape {
        Shape::Plain(s) => {
            for f in s.face_iter() {
                *faces.entry(kind(&f.surface())).or_insert(0) += 1;
            }
            let valid = Solid::try_new(s.boundaries().clone()).is_ok();
            (s.triangulation(MESH_TOL * shapes::size(s)).to_polygon(), valid)
        }
        Shape::Rounded(s) => {
            for f in s.face_iter() {
                *faces.entry(rounded::kind(&f.surface())).or_insert(0) += 1;
            }
            let valid = rounded::Solid::try_new(s.boundaries().clone()).is_ok();
            (s.triangulation(MESH_TOL * rounded::size(s)).to_polygon(), valid)
        }
    };
    write_stl(mesh, out)?;
    Ok(json!({"status": "ok", "measured": "mesh", "faces": faces, "valid": valid}))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some("--version") {
        let rev = env!("TRUCK_REV");
        println!("truck (git ricosjp/truck {rev}), runner {}", env!("CARGO_PKG_VERSION"));
        return;
    }
    let (Some(case), Some(out)) = (args.get(1), args.get(2)) else {
        eprintln!("usage: brep-bench-truck CASE OUT.stl | --version");
        std::process::exit(2);
    };
    let Some(&(_, build)) = CASES.iter().find(|(id, _)| id == case) else {
        eprintln!("no implementation of `{case}` (src/cases/<category>/<name_snake>.rs)");
        std::process::exit(2);
    };
    let reply = match build().and_then(|shape| answer(&shape, out)) {
        Ok(v) => v,
        Err(Refused(why)) => json!({"status": "refused", "reason": why}),
    };
    println!("{reply}");
}
