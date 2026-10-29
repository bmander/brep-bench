//! The brep-bench adapter for arris (https://github.com/arris-labs/arris).
//!
//!     brep-bench-arris CASE.json OUT.stl
//!
//! Prints one JSON line: the measures of the case's result, or a refusal. Every op is built from
//! arris's public API: its box and cylinder primitives, extrude, revolve (the cone, the ball and
//! the torus are revolved profiles, as arris has no primitive for them), sweep along an arc or a
//! helix, the three Booleans and the constant-radius fillet. An arris error is a refusal: the
//! kernel declined, saying why.
use arris::check::{check, Level};
use arris::geom::{FrameLaw, Path, PathSegment, Profile, ProfileLoop, ProfileSegment, SurfaceKind};
use arris::math::{Axis, Control, Frame, Point2, Point3, UnitVec3, Vec3};
use arris::ops::measure::mass_properties;
use arris::ops::query::{distance, Operand};
use arris::ops::{
    common, cut, extrude, fillet, fuse, primitive_box, primitive_cylinder, revolve, sweep,
};
use arris::topo::{Body, Edge, Model};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::f64::consts::{PI, TAU};

/// Why a case is not built: arris (or this adapter) cannot, and says why.
struct Refused(String);

impl<E: std::fmt::Display> From<E> for Refused {
    fn from(e: E) -> Refused {
        Refused(e.to_string())
    }
}

type Built<T> = Result<T, Refused>;

const NONE: &Control<'static> = &Control::NONE;

fn num(op: &Value, key: &str) -> f64 {
    op[key]
        .as_f64()
        .unwrap_or_else(|| panic!("`{key}` is a number"))
}

fn v3(j: &Value) -> Vec3 {
    let a = j.as_array().expect("a point is an array");
    Vec3::new(
        a[0].as_f64().unwrap(),
        a[1].as_f64().unwrap(),
        a[2].as_f64().unwrap(),
    )
}

fn point(op: &Value, key: &str) -> Point3 {
    Point3::from(v3(&op[key]))
}

/// The direction square to `axis` that a frame about it takes for its `x`: Open CASCADE's rule,
/// so for `axis` = z it is x, as SPEC.md places a helix's start.
fn square_to(axis: Vec3) -> Built<Vec3> {
    Ok(Frame::from_z(Point3::origin(), axis)?.x().into_inner())
}

/// The plane through `origin` holding `axis`, its `u` along `radial` and its `v` along `axis`.
fn meridian_plane(origin: Point3, axis: Vec3, radial: Vec3) -> Built<Frame> {
    Ok(Frame::new(origin, radial.cross(&axis), radial)?)
}

/// A closed chain of straight segments through `points` in (u, v).
fn polyline_loop(points: &[Point2]) -> ProfileLoop {
    let mut segments: Vec<_> = points[1..]
        .iter()
        .map(|&p| ProfileSegment::LineTo(p))
        .collect();
    segments.push(ProfileSegment::LineTo(points[0]));
    ProfileLoop::Path {
        start: points[0],
        segments,
    }
}

/// A planar polygon given in space as a profile: its plane by Newell's rule, its `x` along the
/// first side.
fn polygon(op: &Value) -> Built<Profile> {
    let points: Vec<Point3> = op["polygon"]
        .as_array()
        .expect("polygon")
        .iter()
        .map(|p| Point3::from(v3(p)))
        .collect();
    let n = points.len();
    if n < 3 {
        return Err(Refused("a polygon of fewer than three points".into()));
    }
    let mut normal = Vec3::zeros();
    for k in 0..n {
        normal += points[k].coords.cross(&points[(k + 1) % n].coords);
    }
    let plane = Frame::new(points[0], normal, points[1] - points[0])?;
    let (x, y) = (plane.x().into_inner(), plane.y().into_inner());
    let uv: Vec<Point2> = points
        .iter()
        .map(|p| Point2::new((p - points[0]).dot(&x), (p - points[0]).dot(&y)))
        .collect();
    Ok(Profile {
        plane,
        outer: polyline_loop(&uv),
        holes: Vec::new(),
    })
}

fn cone(m: &mut Model, op: &Value) -> Built<Body> {
    let (base, axis) = (point(op, "base"), v3(&op["axis"]));
    let (r1, r2, h) = (num(op, "radius1"), num(op, "radius2"), num(op, "height"));
    let plane = meridian_plane(base, axis, square_to(axis)?)?;
    let p = Point2::new;
    let outer = polyline_loop(&[p(0., 0.), p(r1, 0.), p(r2, h), p(0., h)]);
    let profile = Profile {
        plane,
        outer,
        holes: Vec::new(),
    };
    Ok(revolve(m, &profile, Axis::new(base, axis)?, TAU, NONE)?.0)
}

/// The half disc of `radius` about the axis' segment through `centre`, its arc bulging to `u > 0`.
fn half_disc(radius: f64) -> ProfileLoop {
    ProfileLoop::Path {
        start: Point2::new(0., -radius),
        segments: vec![
            ProfileSegment::LineTo(Point2::new(0., radius)),
            ProfileSegment::ArcTo {
                to: Point2::new(0., -radius),
                via: Point2::new(radius, 0.),
            },
        ],
    }
}

/// The ball, the half disc turned a full turn about the axis through its centre.
fn ball(m: &mut Model, centre: Point3, radius: f64) -> Built<Body> {
    let axis = Vec3::z();
    let plane = meridian_plane(centre, axis, square_to(axis)?)?;
    let profile = Profile {
        plane,
        outer: half_disc(radius),
        holes: Vec::new(),
    };
    Ok(revolve(m, &profile, Axis::new(centre, axis)?, TAU, NONE)?.0)
}

fn torus(m: &mut Model, op: &Value) -> Built<Body> {
    let (centre, axis) = (point(op, "center"), v3(&op["axis"]));
    let plane = meridian_plane(centre, axis, square_to(axis)?)?;
    let outer = ProfileLoop::Circle {
        center: Point2::new(num(op, "major"), 0.),
        radius: num(op, "minor"),
    };
    let profile = Profile {
        plane,
        outer,
        holes: Vec::new(),
    };
    Ok(revolve(m, &profile, Axis::new(centre, axis)?, TAU, NONE)?.0)
}

/// The disc square to a helix (an arc when its pitch is zero), carried along it, its ends flat
/// or rounded by a ball at each end.
fn sweep_circle(m: &mut Model, op: &Value) -> Built<Body> {
    let r = num(op, "radius");
    let h = &op["helix"];
    let (centre, axis) = (point(h, "center"), v3(&h["axis"]).normalize());
    let (radius, pitch, turns) = (num(h, "radius"), num(h, "pitch"), num(h, "turns"));
    let x = square_to(axis)?;
    let y = axis.cross(&x);
    let angle = TAU * turns;
    // the point of the path at a turn of `t` radians
    let at = |t: f64| centre + radius * (t.cos() * x + t.sin() * y) + axis * (pitch * t / TAU);
    let start = at(0.);
    let segments = if pitch == 0. {
        // the arc in pieces of at most half a turn, each exact
        let pieces = (2. * turns).ceil().max(1.);
        let step = angle / pieces;
        (0..pieces as usize)
            .map(|k| {
                let t = step * k as f64;
                PathSegment::ArcTo {
                    via: at(t + 0.5 * step),
                    to: at(t + step),
                }
            })
            .collect()
    } else {
        vec![PathSegment::Helix {
            axis: Axis::new(centre, axis)?,
            pitch,
            angle,
        }]
    };
    let path = Path { start, segments };
    let tangent = |t: f64| radius * (t.cos() * y - t.sin() * x) + axis * (pitch / TAU);
    let plane = Frame::new(start, tangent(0.), x)?;
    let profile = Profile {
        plane,
        outer: ProfileLoop::Circle {
            center: Point2::origin(),
            radius: r,
        },
        holes: Vec::new(),
    };
    let law = if pitch == 0. {
        FrameLaw::RotationMinimizing
    } else {
        FrameLaw::FixedBinormal(UnitVec3::new_normalize(axis))
    };
    let pipe = sweep(m, &profile, &path, law, NONE)?.0;
    match op["ends"].as_str().unwrap_or("flat") {
        "flat" => Ok(pipe),
        "round" => {
            let mut body = pipe;
            for end in [start, at(angle)] {
                let b = ball(m, end, r)?;
                body = fuse(m, body, b, NONE)?.0;
            }
            Ok(body)
        }
        other => Err(Refused(format!("no `{other}` ends in this adapter"))),
    }
}

/// Whether `e` is a seam: every use of it in one face, where a closed face meets itself rather
/// than another face. A seam is where a parametrisation closes, not an edge of the solid.
fn seam(m: &Model, e: Edge) -> Built<bool> {
    let uses = m.edge_uses(e.id)?;
    Ok(uses.windows(2).all(|w| w[0].face == w[1].face))
}

/// The edges of `body` a selector names: every edge, or for each point the edge nearest it by
/// arris's own distance query. Seams are not edges of the solid and are never selected: a
/// cylinder's seam runs through the point that names its rim.
fn selected(m: &Model, body: Body, sel: &Value) -> Built<Vec<Edge>> {
    let mut edges = Vec::new();
    for e in m.edges(body)? {
        if !seam(m, e)? {
            edges.push(e);
        }
    }
    if sel["all"].as_bool() == Some(true) {
        return Ok(edges);
    }
    let mut out = Vec::new();
    for p in sel["nearest"].as_array().expect("an edge selector") {
        let p = Point3::from(v3(p));
        let mut best: Option<(f64, Edge)> = None;
        for &e in &edges {
            let d = distance(m, Operand::Point(p), Operand::Shape(e.shape()), NONE)?.distance;
            if best.is_none_or(|(b, _)| d < b) {
                best = Some((d, e));
            }
        }
        out.push(
            best.ok_or_else(|| Refused("a body with no edges".into()))?
                .1,
        );
    }
    Ok(out)
}

/// The solid one op builds from the solids before it.
fn one(m: &mut Model, shapes: &BTreeMap<String, Body>, op: &Value) -> Built<Body> {
    let take = |k: &str| shapes[op[k].as_str().expect(k)];
    Ok(match op["op"].as_str().expect("op") {
        "box" => primitive_box(m, point(op, "min"), point(op, "max"), NONE)?.0,
        "cylinder" => {
            let axis = Axis::new(point(op, "base"), v3(&op["axis"]))?;
            primitive_cylinder(m, axis, num(op, "radius"), num(op, "height"), NONE)?.0
        }
        "cone" => cone(m, op)?,
        "sphere" => ball(m, point(op, "center"), num(op, "radius"))?,
        "torus" => torus(m, op)?,
        "extrude" => {
            let v = v3(&op["vector"]);
            extrude(m, &polygon(op)?, v, v.norm(), NONE)?.0
        }
        "revolve" => {
            let axis = Axis::new(point(op, "axis_origin"), v3(&op["axis"]))?;
            let angle = num(op, "angle") * PI / 180.;
            revolve(m, &polygon(op)?, axis, angle, NONE)?.0
        }
        "sweep_circle" => sweep_circle(m, op)?,
        "union" => fuse(m, take("a"), take("b"), NONE)?.0,
        "cut" => cut(m, take("a"), take("b"), NONE)?.0,
        "common" => common(m, take("a"), take("b"), NONE)?.0,
        "fillet" => {
            let target = take("target");
            let edges = selected(m, target, &op["edges"])?;
            fillet(m, target, &edges, num(op, "radius"), NONE)?.0
        }
        other => return Err(Refused(format!("no `{other}` in this adapter"))),
    })
}

fn build(m: &mut Model, case: &Value) -> Built<Body> {
    let mut shapes: BTreeMap<String, Body> = BTreeMap::new();
    for op in case["ops"].as_array().expect("ops") {
        let (kind, id) = (
            op["op"].as_str().expect("op"),
            op["id"].as_str().expect("id"),
        );
        let body =
            one(m, &shapes, op).map_err(|Refused(why)| Refused(format!("{kind} `{id}`: {why}")))?;
        shapes.insert(id.to_string(), body);
    }
    Ok(shapes[case["result"].as_str().expect("result")])
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
fn report(m: &Model, body: Body, out: &str) -> Built<Value> {
    let props = mass_properties(m, body, NONE)?;
    let mut faces: BTreeMap<&str, usize> = BTreeMap::new();
    for f in m.faces(body)? {
        let surface = m.surface(m.face(f.id)?.surface())?;
        *faces.entry(kind(surface.kind())).or_default() += 1;
    }
    let valid = check(m, body, Level::Full).is_ok();
    // a chord of a fraction of the body's thickness (volume over area: half a tube's radius), so a
    // thin pipe is meshed as closely as a block, well inside the harness' 2e-2 on the volume
    let chord = 5e-3 * props.volume / props.area;
    let mesh =
        arris::mesh::tessellate(m, body, chord, NONE).map_err(|e| format!("not meshed: {e}"))?;
    let bytes = arris::io::stl::write_binary(std::slice::from_ref(&mesh), "arris")?;
    std::fs::write(out, bytes).expect("the STL file writes");
    Ok(json!({
        "status": "ok", "volume": props.volume, "area": props.area, "faces": faces, "valid": valid,
    }))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some("--version") {
        println!(
            "arris {} (adapter {})",
            arris_version(),
            env!("CARGO_PKG_VERSION")
        );
        return;
    }
    let text = std::fs::read_to_string(&args[1]).expect("the case file reads");
    let case: Value = serde_json::from_str(&text).expect("the case is JSON");
    let mut m = Model::default();
    let reply = match build(&mut m, &case).and_then(|b| report(&m, b, &args[2])) {
        Ok(reply) => reply,
        Err(Refused(why)) => json!({"status": "refused", "reason": why}),
    };
    println!("{reply}");
}

/// The version of arris this adapter was built against, from the lock file read at build time.
fn arris_version() -> &'static str {
    let lock = include_str!("../Cargo.lock");
    lock.split("[[package]]")
        .find(|p| p.contains("name = \"arris\"\n"))
        .and_then(|p| p.lines().find_map(|l| l.strip_prefix("version = \"")))
        .map_or("unknown", |v| v.trim_end_matches('"'))
}
