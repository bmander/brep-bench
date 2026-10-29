//! The brep-bench adapter for truck (https://github.com/ricosjp/truck).
//!
//!     brep-bench-truck CASE.json OUT.stl
//!
//! Prints one JSON line. truck measures only meshes, so a built case answers
//! `"measured": "mesh"` and the harness takes the volume and area from the triangles written to
//! `OUT.stl`: truck's own tessellation of the solid, welded by truck's own filter. A case truck
//! cannot build is a refusal naming what is missing or which truck operation gave up; a panic
//! inside truck is left to end the process.
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::f64::consts::PI;
use truck_meshalgo::prelude::*;
use truck_modeling::{builder, BoundingBox, Face, Rad, Shell, Solid, Surface, Wire};
use truck_modeling::{EuclideanSpace, InnerSpace, ParametricSurface3D, Point3, Vector3};

mod rounded;

/// Why a case is not built: the kernel cannot, or one of its operations declined.
pub struct Refused(String);

impl<T: Into<String>> From<T> for Refused {
    fn from(s: T) -> Refused {
        Refused(s.into())
    }
}

type Built<T> = Result<T, Refused>;

/// A solid as the ops hand it on: one of truck-modeling's, or one with an edge rounded, which
/// carries the fillet's own geometry types (see `rounded`).
enum Body {
    Plain(Solid),
    Rounded(rounded::Solid),
}

/// The tolerance of a Boolean, relative to the size of the smaller operand: truck-shapeops meshes
/// both operands to it to find where they meet and fits the curves it finds to it. This is the
/// scale of truck's own example (0.05 on a unit cube). Which cases truck builds changes with it in
/// no orderly way; no other tolerance from 1e-2 down to 2e-4 builds more of them.
const BOOLEAN_TOL: f64 = 3e-2;
/// The tolerance of the written mesh, relative to the size of the solid. truck's tessellation
/// keeps each curve and surface within it, and the error of the volume and area grows linearly
/// with it: at this one a torus is within 5e-4 of both, half the 1e-3 the harness asks.
const MESH_TOL: f64 = 4e-5;
/// The tolerance a fillet's surface is approximated to, relative to the size of the target.
const FILLET_TOL: f64 = 1e-4;

fn num(op: &Value, key: &str) -> Built<f64> {
    op.get(key)
        .and_then(Value::as_f64)
        .ok_or_else(|| format!("no number `{key}`").into())
}

fn vec3(j: &Value) -> Built<Vector3> {
    match j
        .as_array()
        .map(|a| a.iter().map(Value::as_f64).collect::<Vec<_>>())
    {
        Some(a) if a.len() == 3 && a.iter().all(Option::is_some) => {
            Ok(Vector3::new(a[0].unwrap(), a[1].unwrap(), a[2].unwrap()))
        }
        _ => Err("a point that is not three numbers".into()),
    }
}

fn to_point(v: Vector3) -> Point3 {
    Point3::from_vec(v)
}

fn point(op: &Value, key: &str) -> Built<Point3> {
    vec3(op.get(key).ok_or_else(|| format!("no point `{key}`"))?).map(to_point)
}

fn unit(op: &Value, key: &str) -> Built<Vector3> {
    let v = vec3(op.get(key).ok_or_else(|| format!("no vector `{key}`"))?)?;
    if v.magnitude() == 0.0 {
        return Err(format!("`{key}` is the zero vector").into());
    }
    Ok(v.normalize())
}

fn polygon(op: &Value) -> Built<Vec<Point3>> {
    let pts = op
        .get("polygon")
        .and_then(Value::as_array)
        .ok_or("no `polygon`")?;
    pts.iter().map(|p| vec3(p).map(to_point)).collect()
}

/// The fixed direction square to `a` that every adapter starts a circle from: x for a = z.
fn square_to(a: Vector3) -> Vector3 {
    let e = if a.x.abs() <= a.y.abs() && a.x.abs() <= a.z.abs() {
        Vector3::unit_x()
    } else if a.y.abs() <= a.z.abs() {
        Vector3::unit_y()
    } else {
        Vector3::unit_z()
    };
    (e - a * e.dot(a)).normalize()
}

/// `face` turned over, if need be, so that its normal points along `dir`.
fn facing(face: Face, dir: Vector3) -> Face {
    match face.oriented_surface().normal(0.0, 0.0).dot(dir) < 0.0 {
        true => face.inverse(),
        false => face,
    }
}

fn attached(wire: Wire) -> Built<Face> {
    builder::try_attach_plane(&[wire])
        .map_err(|e| format!("truck-modeling `try_attach_plane` failed: {e}").into())
}

/// The planar face bounded by a closed polygon.
fn polygon_face(points: &[Point3]) -> Built<Face> {
    if points.len() < 3 {
        return Err("a polygon of fewer than three points".into());
    }
    let v = builder::vertices(points.iter().copied());
    let n = v.len();
    attached(
        (0..n)
            .map(|k| builder::line(&v[k], &v[(k + 1) % n]))
            .collect(),
    )
}

/// The disc of radius `r` about `centre`, square to `normal`, its normal along `normal`.
fn disc(centre: Point3, normal: Vector3, r: f64) -> Built<Face> {
    let start = builder::vertex(centre + square_to(normal) * r);
    let circle: Wire = builder::rsweep(&start, centre, normal, Rad(2.0 * PI), 4);
    Ok(facing(attached(circle)?, normal))
}

fn solid(shell: Shell, what: &str) -> Built<Solid> {
    Solid::try_new(vec![shell])
        .map_err(|e| format!("the {what} shell is not a solid in truck-topology: {e}").into())
}

fn cuboid(lo: Point3, hi: Point3) -> Built<Solid> {
    let d = hi - lo;
    if d.x <= 0.0 || d.y <= 0.0 || d.z <= 0.0 {
        return Err("a box with no volume".into());
    }
    let v = builder::vertex(lo);
    let e = builder::tsweep(&v, Vector3::new(d.x, 0.0, 0.0));
    let f = builder::tsweep(&e, Vector3::new(0.0, d.y, 0.0));
    Ok(builder::tsweep(&f, Vector3::new(0.0, 0.0, d.z)))
}

fn cylinder(op: &Value) -> Built<Solid> {
    let (base, a) = (point(op, "base")?, unit(op, "axis")?);
    Ok(builder::tsweep(
        &disc(base, a, num(op, "radius")?)?,
        a * num(op, "height")?,
    ))
}

/// The solid swept by turning a meridian, which runs from a point on the axis back to the axis,
/// once about `axis` (unit) through its first point: `builder::cone`, which closes the ends that
/// lie on the axis. The meridian runs against `axis`, or the solid comes out inside out.
fn turned(meridian: Wire, axis: Vector3, what: &str) -> Built<Solid> {
    solid(builder::cone(&meridian, axis, Rad(2.0 * PI), 4), what)
}

fn cone(op: &Value) -> Built<Solid> {
    let (base, a) = (point(op, "base")?, unit(op, "axis")?);
    let (r1, r2, h) = (num(op, "radius1")?, num(op, "radius2")?, num(op, "height")?);
    let x = square_to(a);
    let mut pts = vec![base + a * h];
    if r2 > 0.0 {
        pts.push(base + a * h + x * r2);
    }
    pts.extend([base + x * r1, base]);
    let v = builder::vertices(pts);
    turned(
        v.windows(2).map(|w| builder::line(&w[0], &w[1])).collect(),
        a,
        "cone",
    )
}

fn sphere(centre: Point3, r: f64) -> Built<Solid> {
    let a = Vector3::unit_z();
    // the meridian from pole to pole in three arcs, as truck's own sphere example draws it, so
    // that no edge runs round the equator
    let pole = builder::vertex(centre + a * r);
    let meridian: Wire = builder::rsweep(&pole, centre, a.cross(square_to(a)), Rad(PI), 3);
    turned(meridian, a, "sphere")
}

fn torus(op: &Value) -> Built<Solid> {
    let (c, a) = (point(op, "center")?, unit(op, "axis")?);
    let (major, minor) = (num(op, "major")?, num(op, "minor")?);
    let x = square_to(a);
    let at = c + x * major;
    let start = builder::vertex(at + a * minor);
    let section: Wire = builder::rsweep(&start, at, a.cross(x), Rad(2.0 * PI), 4);
    solid(builder::rsweep(&section, c, a, Rad(2.0 * PI), 4), "torus")
}

fn extrude(op: &Value) -> Built<Solid> {
    let v = vec3(op.get("vector").ok_or("no `vector`")?)?;
    let face = facing(polygon_face(&polygon(op)?)?, v);
    Ok(builder::tsweep(&face, v))
}

/// `face` turned by `angle` (radians, right-handed) about the line through `origin` along `axis`
/// (unit), in quarter turns at most.
fn revolved(face: Face, origin: Point3, axis: Vector3, angle: f64) -> Solid {
    // the face's normal must point the way its points move, or the solid comes out inside out
    let pts: Vec<Vector3> = face.vertex_iter().map(|v| v.point().to_vec()).collect();
    let centroid = to_point(pts.iter().sum::<Vector3>() / pts.len() as f64);
    let face = facing(face, axis.cross(centroid - origin));
    let quarters = (angle.abs() / (PI / 2.0)).ceil() as usize;
    builder::rsweep(&face, origin, axis, Rad(angle), quarters.clamp(1, 4))
}

fn revolve(op: &Value) -> Built<Solid> {
    let face = polygon_face(&polygon(op)?)?;
    let angle = num(op, "angle")?.to_radians();
    Ok(revolved(
        face,
        point(op, "axis_origin")?,
        unit(op, "axis")?,
        angle,
    ))
}

fn sweep_circle(op: &Value) -> Built<Solid> {
    let r = num(op, "radius")?;
    let h = op.get("helix").ok_or("a sweep with no `helix`")?;
    let (c, a) = (point(h, "center")?, unit(h, "axis")?);
    let (radius, pitch, turns) = (num(h, "radius")?, num(h, "pitch")?, num(h, "turns")?);
    if pitch != 0.0 {
        return Err(
            "no helix in truck: truck-geometry has no helical curve, and truck-modeling \
                    sweeps only by translation (tsweep) or rotation (rsweep), never along a path"
                .into(),
        );
    }
    // with no pitch the path is an arc, and the pipe the disc square to it turned about the axis
    let x = square_to(a);
    let start = c + x * radius;
    let pipe = revolved(disc(start, a.cross(x), r)?, c, a, 2.0 * PI * turns);
    match op.get("ends").and_then(Value::as_str).unwrap_or("flat") {
        "flat" => Ok(pipe),
        "round" => {
            let end = builder::rotated(&builder::vertex(start), c, a, Rad(2.0 * PI * turns));
            let pipe = boolean("union", &pipe, &sphere(start, r)?)?;
            boolean("union", &pipe, &sphere(end.point(), r)?)
        }
        other => Err(format!("no ends `{other}`").into()),
    }
}

/// The diagonal of the bounding box of the solid's vertices. (Not of points along its edges: a
/// truck intersection curve can panic where it is evaluated.)
fn size_of(s: &Solid) -> f64 {
    let b: BoundingBox<Point3> = s.vertex_iter().map(|v| v.point()).collect();
    b.diameter()
}

/// The Boolean of two solids: truck-shapeops' `or`, or its `and`, of `b` turned inside out for a
/// cut.
fn boolean(kind: &str, a: &Solid, b: &Solid) -> Built<Solid> {
    let size = size_of(a).min(size_of(b));
    let mut not_b = b.clone();
    let (name, b) = match kind {
        "union" => ("or", b),
        "common" => ("and", b),
        _ => {
            not_b.not();
            ("and", &not_b)
        }
    };
    let out = match name {
        "or" => truck_shapeops::or(a, b, BOOLEAN_TOL * size),
        _ => truck_shapeops::and(a, b, BOOLEAN_TOL * size),
    };
    out.ok_or_else(|| format!("truck-shapeops `{name}` returned None").into())
}

fn build(case: &Value) -> Built<Body> {
    let mut shapes: BTreeMap<String, Body> = BTreeMap::new();
    for op in case
        .get("ops")
        .and_then(Value::as_array)
        .ok_or("no `ops`")?
    {
        let kind = op
            .get("op")
            .and_then(Value::as_str)
            .ok_or("an op with no `op`")?;
        let plain = |key: &str| -> Built<&Solid> {
            let id = op
                .get(key)
                .and_then(Value::as_str)
                .ok_or(format!("no `{key}`"))?;
            match shapes.get(id) {
                Some(Body::Plain(s)) => Ok(s),
                Some(Body::Rounded(_)) => Err("a filleted solid as an operand: its fillet \
                                               geometry is outside truck-modeling's types"
                    .into()),
                None => Err(format!("no solid `{id}` built before").into()),
            }
        };
        let body = match kind {
            "box" => Body::Plain(cuboid(point(op, "min")?, point(op, "max")?)?),
            "cylinder" => Body::Plain(cylinder(op)?),
            "cone" => Body::Plain(cone(op)?),
            "sphere" => Body::Plain(sphere(point(op, "center")?, num(op, "radius")?)?),
            "torus" => Body::Plain(torus(op)?),
            "extrude" => Body::Plain(extrude(op)?),
            "revolve" => Body::Plain(revolve(op)?),
            "sweep_circle" => Body::Plain(sweep_circle(op)?),
            "union" | "cut" | "common" => Body::Plain(boolean(kind, plain("a")?, plain("b")?)?),
            "fillet" => {
                let target = plain("target")?;
                let edges = op.get("edges").ok_or("a fillet with no `edges`")?;
                let tol = FILLET_TOL * size_of(target);
                Body::Rounded(rounded::fillet(target, num(op, "radius")?, edges, tol)?)
            }
            other => return Err(format!("no `{other}` in this adapter").into()),
        };
        let id = op
            .get("id")
            .and_then(Value::as_str)
            .ok_or("an op with no `id`")?;
        shapes.insert(id.to_string(), body);
    }
    let result = case
        .get("result")
        .and_then(Value::as_str)
        .ok_or("no `result`")?;
    shapes
        .remove(result)
        .ok_or_else(|| format!("no solid `{result}`").into())
}

fn kind(s: &Surface) -> &'static str {
    match s {
        Surface::Plane(_) => "plane",
        Surface::BSplineSurface(_) | Surface::NurbsSurface(_) => "bspline",
        Surface::RevolutedCurve(_) => "other",
    }
}

/// truck's tessellation of the solid, its faces' meshes joined where they share a vertex
/// (`put_together_same_attrs`, as truck's documentation closes a solid's mesh), as binary STL.
fn write_stl(mut mesh: PolygonMesh, path: &str) -> Built<()> {
    mesh.put_together_same_attrs(TOLERANCE)
        .remove_degenerate_faces()
        .remove_unused_attrs();
    let file = std::fs::File::create(path).map_err(|e| e.to_string())?;
    let mut out = std::io::BufWriter::new(file);
    stl::write(&mesh, &mut out, stl::StlType::Binary)
        .map_err(|e| format!("the STL is not written: {e}").into())
}

fn answer(body: &Body, out: &str) -> Built<Value> {
    let mut faces = BTreeMap::new();
    // `valid` is truck-topology's check of a solid: each shell connected, closed and manifold
    let (mesh, valid) = match body {
        Body::Plain(s) => {
            for f in s.face_iter() {
                *faces.entry(kind(&f.surface())).or_insert(0) += 1;
            }
            let valid = Solid::try_new(s.boundaries().clone()).is_ok();
            (s.triangulation(MESH_TOL * size_of(s)).to_polygon(), valid)
        }
        Body::Rounded(s) => {
            for f in s.face_iter() {
                *faces.entry(rounded::kind(&f.surface())).or_insert(0) += 1;
            }
            let valid = rounded::Solid::try_new(s.boundaries().clone()).is_ok();
            (
                s.triangulation(MESH_TOL * rounded::size_of(s)).to_polygon(),
                valid,
            )
        }
    };
    write_stl(mesh, out)?;
    Ok(json!({"status": "ok", "measured": "mesh", "faces": faces, "valid": valid}))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some("--version") {
        let rev = env!("TRUCK_REV");
        println!(
            "truck (git ricosjp/truck {rev}), adapter {}",
            env!("CARGO_PKG_VERSION")
        );
        return;
    }
    let text = std::fs::read_to_string(&args[1]).expect("the case file reads");
    let case: Value = serde_json::from_str(&text).expect("the case is JSON");
    let reply = match build(&case).and_then(|b| answer(&b, &args[2])) {
        Ok(v) => v,
        Err(Refused(why)) => json!({"status": "refused", "reason": why}),
    };
    println!("{reply}");
}
