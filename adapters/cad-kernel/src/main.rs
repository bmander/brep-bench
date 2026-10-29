//! The brep-bench adapter for cad-kernel (https://github.com/bmander/cad-kernel).
//!
//!     brep-bench-cad-kernel CASE.json OUT.stl
//!
//! Prints one JSON line: the measures of the case's result, or a refusal naming what the kernel's
//! public API does not offer. A kernel error is a refusal: the kernel declined, saying why.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{ball_section, canal, disc, prism, revolve, Profile, ProfileEdge};
use cad_kernel::geom::{Frame, Surface, V};
use cad_kernel::mesh::mesh;
use cad_kernel::props::{area, volume};
use cad_kernel::topo::Brep;
use cad_kernel::util::json::{parse, Json};
use cad_kernel::util::space::{add, cross, dot, norm, scale, sub, unit};
use std::collections::BTreeMap;
use std::f64::consts::TAU;
use std::io::Write;

enum Failure {
    Refused(String),
}

impl From<String> for Failure {
    fn from(s: String) -> Failure { Failure::Refused(s) }
}

fn v3(j: &Json) -> V {
    let a = j.arr();
    [a[0].as_f64(), a[1].as_f64(), a[2].as_f64()]
}

fn num(op: &Json, key: &str) -> f64 { op.get(key).map_or(0., Json::as_f64) }

fn point(op: &Json, key: &str) -> V { v3(op.get(key).expect(key)) }

/// A unit vector square to `a`.
fn square_to(a: V) -> V { Frame::about([0.; 3], a).x }

/// A closed polygon as a profile, its normal by Newell's rule.
fn polygon(points: &[V]) -> Result<Profile, String> {
    let n = points.len();
    if n < 3 { return Err("a polygon of fewer than three points".into()) }
    let mut normal = [0.; 3];
    for k in 0..n {
        normal = add(normal, cross(points[k], points[(k + 1) % n]));
    }
    if norm(normal) == 0. { return Err("a polygon enclosing nothing".into()) }
    let edges = (0..n).map(|k| ProfileEdge::Line { a: points[k], b: points[(k + 1) % n] }).collect();
    Ok(Profile { origin: points[0], normal: unit(normal), loops: vec![edges], names: Vec::new() })
}

fn points(op: &Json) -> Vec<V> { op.get("polygon").expect("polygon").arr().iter().map(v3).collect() }

fn boxed(lo: V, hi: V) -> Result<Brep, String> {
    let p = polygon(&[lo, [hi[0], lo[1], lo[2]], [hi[0], hi[1], lo[2]], [lo[0], hi[1], lo[2]]])?;
    let h = hi[2] - lo[2];
    prism(&p, 0., if p.normal[2] > 0. { h } else { -h })
}

/// A profile in the plane through `base` holding `axis`: points at (radius, height) about it.
fn meridian(base: V, axis: V, rz: &[[f64; 2]]) -> Result<Profile, String> {
    let (a, x) = (unit(axis), square_to(axis));
    polygon(&rz.iter().map(|&[r, z]| add(base, add(scale(x, r), scale(a, z)))).collect::<Vec<_>>())
}

fn sweep_circle(op: &Json) -> Result<Brep, Failure> {
    let r = num(op, "radius");
    let h = op.get("helix").ok_or("a sweep with no helix".to_string())?;
    let (centre, axis) = (point(h, "center"), unit(point(h, "axis")));
    let (radius, pitch, turns) = (num(h, "radius"), num(h, "pitch"), num(h, "turns"));
    // where the path starts: radius along the frame's x, as every adapter places it
    let x = Frame::new(centre, axis, square_to(axis)).x;
    let start = add(centre, scale(x, radius));
    let (angle, advance) = (TAU * turns, pitch * turns);
    match op.get("ends").map_or("flat", Json::as_str) {
        "round" => Ok(canal(start, r, centre, axis, angle, advance, "")?),
        "flat" if pitch == 0. => {
            // the disc square to the arc, turned about the axis
            let tangent = cross(axis, x);
            Ok(revolve(&disc(start, tangent, r), centre, axis, angle)?)
        }
        _ => Err(Failure::Refused("no sweep of a section along a helix with flat ends in the public API".into())),
    }
}

fn build(case: &Json) -> Result<Brep, Failure> {
    let mut shapes: BTreeMap<String, Brep> = BTreeMap::new();
    for op in case.get("ops").expect("ops").arr() {
        let kind = op.get("op").expect("op").as_str();
        let take = |k: &str| shapes.get(op.get(k).expect(k).as_str()).cloned().expect(k);
        let b = match kind {
            "box" => boxed(point(op, "min"), point(op, "max"))?,
            "cylinder" => prism(&disc(point(op, "base"), unit(point(op, "axis")), num(op, "radius")),
                                0., num(op, "height"))?,
            "cone" => {
                let (r1, r2, h) = (num(op, "radius1"), num(op, "radius2"), num(op, "height"));
                let p = meridian(point(op, "base"), point(op, "axis"), &[[0., 0.], [r1, 0.], [r2, h], [0., h]])?;
                revolve(&p, point(op, "base"), point(op, "axis"), TAU)?
            }
            "sphere" => {
                let c = point(op, "center");
                let f = Frame::new(c, [0., 1., 0.], [1., 0., 0.]);
                revolve(&ball_section(&f, num(op, "radius")), c, f.y, TAU)?
            }
            "torus" => {
                let (c, a) = (point(op, "center"), unit(point(op, "axis")));
                let x = square_to(a);
                let at = add(c, scale(x, num(op, "major")));
                revolve(&disc(at, cross(a, x), num(op, "minor")), c, a, TAU)?
            }
            "extrude" => {
                let p = polygon(&points(op))?;
                let v = point(op, "vector");
                if norm(cross(unit(v), p.normal)) > 1e-12 {
                    return Err(Failure::Refused("an extrusion oblique to its profile".into()));
                }
                let d = dot(v, p.normal);
                prism(&p, 0., d)?
            }
            "revolve" => {
                let p = polygon(&points(op))?;
                revolve(&p, point(op, "axis_origin"), point(op, "axis"), num(op, "angle").to_radians())?
            }
            "sweep_circle" => sweep_circle(op)?,
            "union" | "cut" | "common" => {
                let how = match kind { "union" => Op::Union, "cut" => Op::Cut, _ => Op::Common };
                combined(&take("a"), &take("b"), how, 0.).map_err(|e| Failure::Refused(e.msg))?
            }
            "fillet" => return Err(Failure::Refused("no edge fillet in the public API".into())),
            other => return Err(Failure::Refused(format!("no `{other}` in this adapter"))),
        };
        shapes.insert(op.get("id").expect("id").as_str().to_string(), b);
    }
    let result = case.get("result").expect("result").as_str();
    Ok(shapes.remove(result).expect("the result is built"))
}

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

fn stl(b: &Brep, path: &str) -> Result<(), String> {
    let m = mesh(b, 1e-4 * b.size(), 0.1)?;
    let mut out = Vec::with_capacity(84 + 50 * m.tris.len());
    out.extend_from_slice(&[0u8; 80]);
    out.extend_from_slice(&(m.tris.len() as u32).to_le_bytes());
    for t in &m.tris {
        let [a, b, c] = t.map(|i| m.pts[i as usize]);
        let n = unit(cross(sub(b, a), sub(c, a)));
        for p in [n, a, b, c] {
            for x in p { out.extend_from_slice(&(x as f32).to_le_bytes()); }
        }
        out.extend_from_slice(&[0u8; 2]);
    }
    std::fs::write(path, out).map_err(|e| e.to_string())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some("--version") {
        println!("cad-kernel (adapter {})", env!("CARGO_PKG_VERSION"));
        return;
    }
    let text = std::fs::read_to_string(&args[1]).expect("the case file reads");
    let case = parse(&text).expect("the case is JSON");
    let reply = match build(&case) {
        Ok(b) => {
            let mut faces = BTreeMap::new();
            for f in &b.faces { *faces.entry(kind(&f.surface)).or_insert(0) += 1; }
            let a: f64 = b.faces.iter().map(|f| area(&b, f)).sum();
            let valid = b.check(cad_kernel::geom::tolerance(b.size())).is_ok();
            let written = stl(&b, &args[2]);
            let faces = faces.iter().map(|(k, n)| format!("\"{k}\":{n}")).collect::<Vec<_>>().join(",");
            match written {
                Ok(()) => format!("{{\"status\":\"ok\",\"volume\":{:e},\"area\":{a:e},\"faces\":{{{faces}}},\"valid\":{valid}}}", volume(&b)),
                Err(e) => format!("{{\"status\":\"refused\",\"reason\":{}}}", Json::Str(format!("not meshed: {e}")).dump(None)),
            }
        }
        Err(Failure::Refused(why)) => format!("{{\"status\":\"refused\",\"reason\":{}}}", Json::Str(why).dump(None)),
    };
    let mut so = std::io::stdout();
    writeln!(so, "{reply}").unwrap();
}
