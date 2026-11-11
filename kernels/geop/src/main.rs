//! The brep-bench runner for geop (https://github.com/geop-cad/geop).
//!
//!     brep-bench-geop CASE OUT.stl [OUT.step]
//!     brep-bench-geop --version
//!
//! A geop part is a program: an ordered list of steps, each an operation with its arguments,
//! referring to what earlier steps built by stable names (`extrude(box,end)`). The case `CASE`
//! (`boolean/napkin-ring`) writes its program, `src/cases/boolean/napkin_ring.rs`, step by step
//! into a [`Bench`], which runs each step as it is added by the runner of every operation geop's
//! editor offers (`geop_cad_base::runner`), as geop's own examples are built. Then the part's
//! solids are measured with the kernel's own tools: their exact volume and area (integrated over
//! the trimmed NURBS faces, `Model::mass_properties` and `Model::solid_area`), their faces (every
//! one a NURBS: geop has no other kind of surface) and its validity check
//! (`validation::validate`). The solids are meshed into OUT.stl by geop's rasterizer, as finely as
//! geop meshes an STL file (its `STL_QUALITY`, refined to each face's curvature), and
//! one line of JSON is printed. Given `OUT.step`, the part's bodies are also written there by
//! geop's own STEP writer (`geop_ops_step::write_step`, AP214, every surface and curve the NURBS
//! geop holds), the product named after the case; where the writer fails, nothing is written and
//! the reply says why in `step_error`. Given `-` for OUT.stl (the harness's timed run), the part
//! is built and its STEP written, and nothing is measured or meshed. A step that fails is a
//! refusal: the kernel declined, saying why. A case may also refuse in its own words, where geop
//! has no operation for what it describes.
//! The runner never reads the case's description.
use geop_core_math::primitives::TriangleFace;
use geop_core_rasterize::rasterize;
use geop_core_topology::validation::{ValidationParameters, validate};
use geop_core_topology::{Model, SolidId};
use geop_ops_model::{NamedModel, STL_QUALITY};
use geop_ops_step::write_step;
use std::collections::HashMap;
use std::process::ExitCode;

#[allow(dead_code)]
mod bench;
#[allow(dead_code)]
mod shared;

pub use bench::{Bench, Refusal};

/// A case's implementation: it writes the case's program into the bench, or says why not.
type Build = fn(&mut Bench) -> Result<(), Refusal>;

/// One module per case file, and `CASES`, written by `build.rs`.
mod cases {
    #[allow(unused_imports)]
    use super::{Bench, Build, Refusal};
    include!(concat!(env!("OUT_DIR"), "/cases.rs"));
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

/// `triangles` as an STL file, each as the rasterizer wound it, which is outward (geop's faces
/// all face along their surface's normal, `S_u × S_v`, out of the solid, and each face is
/// triangulated counter-clockwise in its `(u, v)`).
///
/// The mesh is closed where geop meets two faces along an edge at exactly the same points, bit
/// for bit. Binary STL keeps each corner as `f32`, which is exact for the mesh's topology as long
/// as no two of its distinct points round to the same `f32` point. Where some do (a finely meshed
/// edge, whose samples lie closer than an `f32` tells apart), it is written as ASCII at full
/// `f64` precision instead, so that the file is the mesh geop made, not one merged by rounding.
///
/// geop's own STL writer (`geop_ops_model::stl_bytes`) is not used: it is binary only, and it
/// winds each triangle again by the surface's normals at its corners (`stl::outward`), which turns
/// some of a fillet's triangles inside out.
fn stl(triangles: &[&TriangleFace]) -> Vec<u8> {
    let corners = |t: &TriangleFace| [&t.a, &t.b, &t.c].map(|v| [0, 1, 2].map(|i| v[i].to_f64()));
    let mut rounded: HashMap<[u32; 3], [u64; 3]> = HashMap::new();
    let lossless = triangles.iter().flat_map(|t| corners(t)).all(|p| {
        let bits = p.map(f64::to_bits);
        *rounded.entry(p.map(|x| (x as f32).to_bits())).or_insert(bits) == bits
    });
    if lossless {
        let mut out = Vec::with_capacity(84 + 50 * triangles.len());
        out.extend_from_slice(&[0u8; 80]);
        out.extend_from_slice(&(triangles.len() as u32).to_le_bytes());
        for t in triangles {
            let n = &t.normal;
            for x in [0, 1, 2].map(|i| n[i].to_f64()).into_iter().chain(corners(t).into_iter().flatten()) {
                out.extend_from_slice(&(x as f32).to_le_bytes());
            }
            out.extend_from_slice(&[0u8; 2]);
        }
        return out;
    }
    let mut text = String::with_capacity(200 * triangles.len() + 32);
    text.push_str("solid geop\n");
    for t in triangles {
        text.push_str("facet normal 0 0 0\nouter loop\n");
        for [x, y, z] in corners(t) {
            text.push_str(&format!("vertex {x} {y} {z}\n"));
        }
        text.push_str("endloop\nendfacet\n");
    }
    text.push_str("endsolid geop\n");
    text.into_bytes()
}

/// The volume and area of `solid`, and whether both integrals converged.
fn measures(model: &Model, solid: SolidId) -> Result<(f64, f64, bool), Refusal> {
    // The two integrals are independent, and each costs seconds on a curved solid.
    let (mass, area) = std::thread::scope(|s| {
        let area = s.spawn(|| model.solid_area(solid));
        (model.mass_properties(solid), area.join().expect("the area's thread does not panic"))
    });
    let (mass, (area, area_converged)) = (mass?, area?);
    Ok((mass.volume.to_f64(), area.to_f64(), mass.converged && area_converged))
}

/// The reply for the part built: the measures of its solids, once their triangles are written.
/// Measuring, checking and meshing it are independent, and run side by side. With `out_step`,
/// the part is then written there as STEP, the product called `name`, or the reply says why geop
/// would not (`step_error`). Given `-` for `out`, the harness's timed run, its STEP alone:
/// nothing measured or meshed.
fn measured(
    part: &NamedModel,
    out: &str,
    out_step: Option<&str>,
    name: &str,
) -> Result<String, Refusal> {
    let model = part.topology();
    let solids: Vec<_> = model.solids.keys().copied().collect();
    if solids.is_empty() {
        return Err("the program built no solid".into());
    }
    let step_error = || {
        let Some(path) = out_step else { return String::new() };
        match write_step(part, name) {
            Ok(text) => {
                std::fs::write(path, text).expect("the STEP file writes");
                String::new()
            }
            Err(e) => {
                let why = Refusal::from(e).to_string();
                format!(",\"step_error\":{}", quoted(&why))
            }
        }
    };
    if out == "-" {
        return Ok(format!("{{\"status\":\"ok\"{}}}", step_error()));
    }
    let (measures, valid, raster) = std::thread::scope(|s| {
        let valid = s.spawn(|| validate(&ValidationParameters::default(), model).is_ok());
        let raster = s.spawn(|| rasterize(model, STL_QUALITY));
        let measures: Vec<_> = solids.iter().map(|&solid| measures(model, solid)).collect();
        let joined = "the checking and meshing threads do not panic";
        (measures, valid.join().expect(joined), raster.join().expect(joined))
    });
    let (mut volume, mut area, mut converged) = (0.0, 0.0, true);
    for m in measures {
        let (v, a, c) = m?;
        (volume, area, converged) = (volume + v, area + a, converged && c);
    }
    let raster = raster.map_err(|e| format!("not meshed: {}", e.root_message()))?;
    let mut faces = 0;
    let mut triangles: Vec<&TriangleFace> = Vec::new();
    for &solid in &solids {
        for face in model.solid_faces(solid)? {
            faces += 1;
            triangles.extend(raster.faces.get(&face).into_iter().flatten());
        }
    }
    std::fs::write(out, stl(&triangles)).map_err(|e| format!("{out} not written: {e}"))?;
    let valid = valid && converged;
    let step_error = step_error();
    Ok(format!(
        "{{\"status\":\"ok\",\"volume\":{volume:e},\"area\":{area:e},\"faces\":{{\"bspline\":{faces}}},\"valid\":{valid}{step_error}}}"
    ))
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some("--version") {
        println!("geop {} (brep-bench runner {})", env!("KERNEL_VERSION"), env!("CARGO_PKG_VERSION"));
        return ExitCode::SUCCESS;
    }
    let (case, out, out_step) = match &args[..] {
        [_, case, out] => (case, out, None),
        [_, case, out, step] => (case, out, Some(step.as_str())),
        _ => {
            eprintln!("usage: brep-bench-geop CASE OUT.stl [OUT.step]");
            return ExitCode::from(2);
        }
    };
    let Some(&(_, build)) = cases::CASES.iter().find(|(id, _)| id == case) else {
        eprintln!("no implementation of `{case}`");
        return ExitCode::from(2);
    };
    let mut bench = Bench::new();
    let reply = match build(&mut bench).and_then(|()| bench.model()) {
        Ok(part) => {
            // The product is named as geop's editor names it after a program's file: the case's.
            let name = case.rsplit('/').next().unwrap_or(case);
            measured(&part, out, out_step, name).unwrap_or_else(|why| refused(&why.to_string()))
        }
        Err(why) => refused(&why.to_string()),
    };
    println!("{reply}");
    ExitCode::SUCCESS
}
