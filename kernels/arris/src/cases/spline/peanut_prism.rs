//! A prism 10 tall on the xy plane whose section is a peanut: the region inside the closed,
//! uniform, periodic cubic B-spline curve through the twelve control points `CONTROL`.
//!
//! arris's profiles hold lines and circular and elliptic arcs, so `extrude` cannot take the
//! curve. arris does extrude a NURBS curve into its exact surface, `NurbsSurface::extrusion`, and
//! leaves a consumer to build the faces on it: the side and the two ends are assembled by
//! `Builder::assemble` and finished by `ops::build`, which proves the body with arris's checker.
use crate::Refusal;
use arris::geom::{pcurve_on, Curve, Curve2, GeomError, NurbsCurve, NurbsSurface, Surface};
use arris::math::{Control, Frame, Interval, Isometry, Meter, Point2, Point3, UnitVec2, Vec2, Vec3};
use arris::ops::{self, BuildKeys, OpError};
use arris::topo::builder::{
    Assembly, Builder, EdgeKey, EdgeSpec, FaceSpec, UseSpec, VertexKey, VertexSpec,
};
use arris::topo::entity::EdgeGeometry;
use arris::topo::{Body, Model, Orientation};
use std::collections::BTreeMap;

/// Counter-clockwise round the peanut.
const CONTROL: [(f64, f64); 12] = [
    (14.0, 0.0), (12.0, 8.0), (4.0, 6.0), (0.0, -21.0 / 8.0), (-4.0, 6.0), (-12.0, 8.0),
    (-14.0, 0.0), (-12.0, -8.0), (-4.0, -6.0), (0.0, 21.0 / 8.0), (4.0, -6.0), (12.0, -8.0),
];
const HEIGHT: f64 = 10.0;
const DEGREE: usize = 3;

// workaround: no NURBS curve in a profile
pub fn build(m: &mut Model) -> Result<Body, Refusal> {
    let geom = |e: GeomError| Refusal::from(e.to_string());
    let tol = m.precision().tolerance();
    let tolerance = m.precision().default_tolerance;
    // periodic: the first three control points again after the twelfth, on the uniform knots
    // −3 … 15, so the domain is [0, 12], one span per control point
    let points = CONTROL.iter().cycle().take(CONTROL.len() + DEGREE)
        .map(|&(x, y)| Point3::new(x, y, 0.0)).collect::<Vec<_>>();
    let knots = (0..points.len() + DEGREE + 1).map(|i| i as f64 - DEGREE as f64).collect();
    let weights = vec![1.0; points.len()];
    let floor_curve = NurbsCurve::new(DEGREE, knots, points, weights).map_err(geom)?;
    let lid_curve = floor_curve.transformed(&Isometry::from_translation(HEIGHT * Vec3::z()));
    let rise = Interval::new(0.0, HEIGHT).map_err(|e| e.to_string())?;
    // u the curve's parameter, periodic, and v the height: its normal ∂u × ∂v outward round a
    // counter-clockwise curve
    let side = NurbsSurface::extrusion(&floor_curve, Vec3::z(), rise).map_err(geom)?;
    let [round, _] = side.domain();
    let (bottom, top) = (floor_curve.eval(round.lo()).point, lid_curve.eval(round.lo()).point);

    let vertices = vec![
        VertexSpec::New { point: bottom, tolerance },
        VertexSpec::New { point: top, tolerance },
    ];
    let (b, t) = (0, 1);
    let (floor_curve, lid_curve) = (Curve::Nurbs(floor_curve), Curve::Nurbs(lid_curve));
    let seam_curve = Curve::Line { origin: bottom, direction: Vec3::z_axis() };
    let edges = vec![
        EdgeSpec::New {
            geometry: EdgeGeometry::Curve { curve: m.add_curve(floor_curve.clone()), range: round },
            start: VertexKey::New(b),
            end: VertexKey::New(b),
            tolerance,
        },
        EdgeSpec::New {
            geometry: EdgeGeometry::Curve { curve: m.add_curve(lid_curve.clone()), range: round },
            start: VertexKey::New(t),
            end: VertexKey::New(t),
            tolerance,
        },
        EdgeSpec::New {
            geometry: EdgeGeometry::Curve { curve: m.add_curve(seam_curve), range: rise },
            start: VertexKey::New(b),
            end: VertexKey::New(t),
            tolerance,
        },
    ];
    let (rim, lip, seam) = (0, 1, 2);
    let use_of = |m: &mut Model, edge: usize, along: bool, pcurve: Curve2| UseSpec {
        edge: EdgeKey::New(edge),
        orientation: if along { Orientation::Forward } else { Orientation::Reversed },
        pcurve: m.add_curve2(pcurve),
    };
    let line = |(u, v): (f64, f64), along: Vec2| Curve2::Line {
        origin: Point2::new(u, v),
        direction: UnitVec2::new_normalize(along),
    };
    // the side: round the floor, up the seam a period on, back round the lid, down the seam
    let side_loop = vec![
        use_of(m, rim, true, line((0.0, rise.lo()), Vec2::x())),
        use_of(m, seam, true, line((round.hi(), 0.0), Vec2::y())),
        use_of(m, lip, false, line((0.0, rise.hi()), Vec2::x())),
        use_of(m, seam, false, line((round.lo(), 0.0), Vec2::y())),
    ];
    // the ends, each a plane with its normal outward: the floor's rim walked clockwise about z,
    // the lid's counter-clockwise
    let plane = |origin: Point3, outward: Vec3| -> Result<Surface, OpError> {
        Ok(Surface::Plane { frame: Frame::new(origin, outward, Vec3::x())? })
    };
    let floor = plane(Point3::origin(), -Vec3::z())?;
    let lid = plane(Point3::new(0.0, 0.0, HEIGHT), Vec3::z())?;
    let mut meter = Meter::new(&Control::NONE);
    let on_floor = pcurve_on(&floor_curve, round, &floor, tol, &mut meter).map_err(geom)?;
    let on_lid = pcurve_on(&lid_curve, round, &lid, tol, &mut meter).map_err(geom)?;
    let floor_loop = vec![use_of(m, rim, false, on_floor)];
    let lid_loop = vec![use_of(m, lip, true, on_lid)];
    let face = |m: &mut Model, surface: Surface, walk: Vec<UseSpec>| FaceSpec::New {
        surface: m.add_surface(surface),
        orientation: Orientation::Forward,
        loops: vec![walk],
        tolerance,
    };
    let faces = vec![
        face(m, Surface::Nurbs(side), side_loop),
        face(m, floor, floor_loop),
        face(m, lid, lid_loop),
    ];
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
    let (prism, _) = ops::build(m, builder, &keys, &Control::NONE)?;
    Ok(prism)
}

/// Every slot keyed by its place in the assembly.
fn keyed<S: Copy + Ord>(slots: &[S]) -> BTreeMap<S, u64> {
    slots.iter().enumerate().map(|(i, &s)| (s, i as u64)).collect()
}
