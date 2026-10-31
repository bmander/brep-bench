//! A square-to-round transition from the square −20 ≤ x, y ≤ 20 at z = 0 to the circle of
//! radius 15 about the z axis at z = 30: each side of the square joined by a flat triangle to the
//! point of the circle nearest it, and each corner to the quarter of the circle between two such
//! points by a piece of an oblique cone, its apex the corner. The convex hull of the two.
//!
//! arris's `loft` matches its sections' segments by arc length, from an edge to an edge, so it
//! cannot join a side to a point or a corner to an arc; and it has no oblique cone, its `Cone`
//! being a right circular one. So the solid's faces are assembled, as `spline/peanut-prism`'s
//! are, and proven by `ops::build`. Each cone is ruled as arris's ruled loft rules a face: the
//! surface of degree one across, between the quarter circle's own rational quadratic
//! (`NurbsCurve::circle`) and its apex, a row of the net collapsed to the corner, so that every
//! ruling runs straight from the corner to a point of the arc. At the apex the face closes on a
//! degenerate edge, as a revolved cone's does at its tip.
use crate::Refusal;
use arris::geom::{pcurve_on, Curve, Curve2, GeomError, NurbsCurve, NurbsSurface, Surface};
use arris::math::{Control, Frame, Interval, Meter, Point2, Point3, UnitVec2, UnitVec3, Vec2, Vec3};
use arris::ops::{self, BuildKeys, OpError};
use arris::topo::builder::{
    Assembly, Builder, EdgeKey, EdgeSpec, FaceSpec, UseSpec, VertexKey, VertexSpec,
};
use arris::topo::entity::EdgeGeometry;
use arris::topo::{Body, Model, Orientation};
use std::collections::BTreeMap;
use std::f64::consts::FRAC_PI_2;

const HEIGHT: f64 = 30.0;
const HALF: f64 = 20.0;
const RADIUS: f64 = 15.0;

pub fn build(m: &mut Model) -> Result<Body, Refusal> {
    let geom = |e: GeomError| Refusal::from(e.to_string());
    let frame = |origin: Point3, normal: Vec3, x: Vec3| Frame::new(origin, normal, x)
        .map_err(|e| Refusal::from(OpError::from(e)));
    let tol = m.precision().tolerance();
    let tolerance = m.precision().default_tolerance;
    let unit = Interval::new(0.0, 1.0).map_err(|e| e.to_string())?;
    // corner k of the square at 45° + 90°·k about the z axis, (20, 20) first; point k of the
    // circle at 90°·k, (15, 0) first. Corner k's cone spans the arc from point k to point k + 1,
    // and the side from corner k to corner k + 1 rises to point k + 1.
    let corner = |k: usize| {
        let (s, c) = (FRAC_PI_2 * k as f64).sin_cos();
        Point3::new(HALF * (c - s), HALF * (s + c), 0.0)
    };
    let quarter = |k: usize| {
        Interval::new(FRAC_PI_2 * k as f64, FRAC_PI_2 * (k + 1) as f64).map_err(|e| e.to_string())
    };
    let rim = frame(Point3::new(0.0, 0.0, HEIGHT), Vec3::z(), Vec3::x())?;
    let arcs = (0..4)
        .map(|k| NurbsCurve::circle(&rim, RADIUS, quarter(k)?).map_err(geom))
        .collect::<Result<Vec<_>, _>>()?;
    let point = |k: usize| arcs[k % 4].eval(arcs[k % 4].domain().lo()).point;

    let mut vertices = Vec::new();
    for k in 0..4 {
        vertices.push(VertexSpec::New { point: corner(k), tolerance });
    }
    for k in 0..4 {
        vertices.push(VertexSpec::New { point: point(k), tolerance });
    }
    let at_corner = |k: usize| VertexKey::New(k % 4);
    let at_point = |k: usize| VertexKey::New(4 + k % 4);

    let mut edges = Vec::new();
    let mut add = |m: &mut Model, curve: Option<Curve>, range: Interval, start, end| {
        let geometry = match curve {
            Some(curve) => EdgeGeometry::Curve { curve: m.add_curve(curve), range },
            None => EdgeGeometry::Degenerate { range },
        };
        edges.push(EdgeSpec::New { geometry, start, end, tolerance });
        edges.len() - 1
    };
    // the straight edge from `a` to `b` on [0, 1]
    let segment = |a: Point3, b: Point3| {
        NurbsCurve::new(1, vec![0.0, 0.0, 1.0, 1.0], vec![a, b], vec![1.0; 2]).map(Curve::Nurbs)
    };
    let (mut foot, mut arc, mut up_to_this, mut up_to_next, mut apex) =
        (vec![], vec![], vec![], vec![], vec![]);
    for k in 0..4 {
        let (from, to) = (corner(k), corner(k + 1));
        let side = Curve::Line { origin: from, direction: UnitVec3::new_normalize(to - from) };
        let length = Interval::new(0.0, (to - from).norm()).map_err(|e| e.to_string())?;
        let edge = add(m, Some(side.clone()), length, at_corner(k), at_corner(k + 1));
        foot.push((edge, side, length));
        let top = Curve::Nurbs(arcs[k].clone());
        arc.push((add(m, Some(top.clone()), quarter(k)?, at_point(k), at_point(k + 1)), top));
        // the two rulings from corner k, to point k and to point k + 1: the edges where its cone
        // meets the triangles beside it
        let to_this = segment(corner(k), point(k)).map_err(geom)?;
        up_to_this.push((add(m, Some(to_this.clone()), unit, at_corner(k), at_point(k)), to_this));
        let to_next = segment(corner(k), point(k + 1)).map_err(geom)?;
        let edge = add(m, Some(to_next.clone()), unit, at_corner(k), at_point(k + 1));
        up_to_next.push((edge, to_next));
        apex.push(add(m, None, quarter(k)?, at_corner(k), at_corner(k)));
    }

    let use_of = |m: &mut Model, edge: usize, along: bool, pcurve: Curve2| UseSpec {
        edge: EdgeKey::New(edge),
        orientation: if along { Orientation::Forward } else { Orientation::Reversed },
        pcurve: m.add_curve2(pcurve),
    };
    let line = |(u, v): (f64, f64), along: Vec2| Curve2::Line {
        origin: Point2::new(u, v),
        direction: UnitVec2::new_normalize(along),
    };
    let face = |m: &mut Model, surface: Surface, walk: Vec<UseSpec>| FaceSpec::New {
        surface: m.add_surface(surface),
        orientation: Orientation::Forward,
        loops: vec![walk],
        tolerance,
    };
    let mut meter = Meter::new(&Control::NONE);
    let mut on = |curve: &Curve, range: Interval, surface: &Surface| {
        pcurve_on(curve, range, surface, tol, &mut meter).map_err(geom)
    };
    let mut faces = Vec::new();
    for k in 0..4 {
        // the cone: u the arc's parameter, v from the apex (0) to the arc (1), ∂u × ∂v outward
        let a = &arcs[k];
        let points = a.control_points().iter().flat_map(|&p| [corner(k), p]).collect();
        let weights = a.weights().iter().flat_map(|&w| [w, w]).collect();
        let knots = [a.knots().to_vec(), vec![0.0, 0.0, 1.0, 1.0]];
        let cone = NurbsSurface::new([2, 1], knots, points, weights).map_err(geom)?;
        let [lo, hi] = [quarter(k)?.lo(), quarter(k)?.hi()];
        // counter-clockwise in (u, v): along the apex, up the ruling to point k + 1, back along
        // the arc, down the ruling from point k
        let walk = vec![
            use_of(m, apex[k], true, line((0.0, 0.0), Vec2::x())),
            use_of(m, up_to_next[k].0, true, line((hi, 0.0), Vec2::y())),
            use_of(m, arc[k].0, false, line((0.0, 1.0), Vec2::x())),
            use_of(m, up_to_this[k].0, false, line((lo, 0.0), Vec2::y())),
        ];
        faces.push(face(m, Surface::Nurbs(cone), walk));

        // the triangle from the side from corner k to corner k + 1 up to point k + 1, walked
        // that way round, counter-clockwise seen from outside
        let (a, b, c) = (corner(k), corner(k + 1), point(k + 1));
        let outward = (b - a).cross(&(c - b));
        let plane = Surface::Plane { frame: frame(a, outward, b - a)? };
        let (side, curve, length) = &foot[k];
        let (rise, rise_curve) = &up_to_this[(k + 1) % 4];
        let (fall, fall_curve) = &up_to_next[k];
        let walk = vec![
            use_of(m, *side, true, on(curve, *length, &plane)?),
            use_of(m, *rise, true, on(rise_curve, unit, &plane)?),
            use_of(m, *fall, false, on(fall_curve, unit, &plane)?),
        ];
        faces.push(face(m, plane, walk));
    }
    // the floor, walked clockwise about +z (its outward normal is −z), and the round top
    let floor = Surface::Plane { frame: frame(Point3::origin(), -Vec3::z(), Vec3::x())? };
    let mut walk = Vec::new();
    for (side, curve, length) in foot.iter().rev() {
        walk.push(use_of(m, *side, false, on(curve, *length, &floor)?));
    }
    faces.push(face(m, floor, walk));
    let top = Surface::Plane { frame: rim };
    let mut walk = Vec::new();
    for (k, (edge, curve)) in arc.iter().enumerate() {
        walk.push(use_of(m, *edge, true, on(curve, quarter(k)?, &top)?));
    }
    faces.push(face(m, top, walk));

    let (builder, slots) =
        Builder::assemble(m, tolerance, Assembly { vertices, edges, shells: vec![faces] })
            .map_err(OpError::from)?;
    let keys = BuildKeys {
        namespace: 0,
        vertices: keyed(&slots.vertices),
        edges: keyed(&slots.edges),
        faces: keyed(&slots.faces[0]),
        shells: vec![0],
        body: 0,
    };
    let (hull, _) = ops::build(m, builder, &keys, &Control::NONE)?;
    Ok(hull)
}

/// Every slot keyed by its place in the assembly.
fn keyed<S: Copy + Ord>(slots: &[S]) -> BTreeMap<S, u64> {
    slots.iter().enumerate().map(|(i, &s)| (s, i as u64)).collect()
}
