//! A dome: the region in the xz plane between the z axis, the x axis out to (9, 0, 0) and the
//! cubic Bézier curve through the control points `CONTROL`, turned a full turn about z.
//!
//! arris's profiles hold lines and circular and elliptic arcs, so `revolve` cannot take the
//! curve. arris does turn a NURBS curve into its exact surface of revolution,
//! `NurbsSurface::revolution`, and leaves a consumer to build the faces on it: the dome and its
//! base are assembled by `Builder::assemble` and finished by `ops::build`, which proves the body
//! with arris's checker.
use crate::Refusal;
use std::collections::BTreeMap;
use arris::geom::{pcurve_on, Curve, Curve2, GeomError, NurbsCurve, NurbsSurface, Surface};
use arris::math::{Control, Frame, Interval, Meter, Point2, Point3, UnitVec2, Vec2, Vec3};
use arris::ops::{self, BuildKeys, OpError};
use arris::topo::builder::{
    Assembly, Builder, EdgeKey, EdgeSpec, FaceSpec, UseSpec, VertexKey, VertexSpec,
};
use arris::topo::entity::EdgeGeometry;
use arris::topo::{Body, Model, Orientation};

const CONTROL: [[f64; 3]; 4] =
    [[9.0, 0.0, 0.0], [12.0, 0.0, 4.0], [9.0, 0.0, 10.0], [0.0, 0.0, 10.0]];

/// The solid turned a full turn about the z axis from the region in the xz plane bounded by
/// `meridian`, a curve from a point on the plane's +x side to a point on the axis, the axis, and
/// the segment square to the axis from the axis to the meridian's start: a domed face, closing
/// over a pole where the meridian meets the axis, standing on a flat disc.
pub fn turned(m: &mut Model, meridian: NurbsCurve) -> Result<Body, Refusal> {
    let geom = |e: GeomError| Refusal::from(e.to_string());
    let tol = m.precision().tolerance();
    let tolerance = m.precision().default_tolerance;
    let span = meridian.domain();
    let (rim, pole) = (meridian.eval(span.lo()).point, meridian.eval(span.hi()).point);
    let dome =
        NurbsSurface::revolution(&meridian, Point3::origin(), Vec3::z_axis(), Interval::TURN)
            .map_err(geom)?;
    // u is the angle about z from the xz plane, v the meridian's own parameter: the rim at v's
    // least, the pole (a collapsed row) at its greatest, the meridian itself the seam at u = 0
    // and again at u = 2π
    let [turn, _] = dome.domain();
    let base = Frame::world().with_origin(Point3::new(0.0, 0.0, rim.z));
    // the rim: the circle the meridian's start turns, as the surface's own isocurve, so its
    // parameter is u
    let circle = NurbsCurve::circle(&base, rim.x, turn).map_err(geom)?;

    let vertices = vec![
        VertexSpec::New { point: rim, tolerance },
        VertexSpec::New { point: pole, tolerance },
    ];
    let (r, p) = (0, 1);
    let rim_curve = Curve::Nurbs(circle);
    let seam_curve = Curve::Nurbs(meridian);
    let edges = vec![
        EdgeSpec::New {
            geometry: EdgeGeometry::Curve { curve: m.add_curve(rim_curve.clone()), range: turn },
            start: VertexKey::New(r),
            end: VertexKey::New(r),
            tolerance,
        },
        EdgeSpec::New {
            geometry: EdgeGeometry::Curve { curve: m.add_curve(seam_curve), range: span },
            start: VertexKey::New(r),
            end: VertexKey::New(p),
            tolerance,
        },
        EdgeSpec::New {
            geometry: EdgeGeometry::Degenerate { range: turn },
            start: VertexKey::New(p),
            end: VertexKey::New(p),
            tolerance,
        },
    ];
    let (rim_edge, seam, apex) = (0, 1, 2);
    let line = |origin: (f64, f64), along: Vec2| Curve2::Line {
        origin: Point2::new(origin.0, origin.1),
        direction: UnitVec2::new_normalize(along),
    };
    let use_of = |m: &mut Model, edge: usize, along: bool, pcurve: Curve2| UseSpec {
        edge: EdgeKey::New(edge),
        orientation: if along { Orientation::Forward } else { Orientation::Reversed },
        pcurve: m.add_curve2(pcurve),
    };

    // the dome, its normal ∂u × ∂v outward: round the rim, up the seam at u = 2π, back along the
    // pole, down the seam at u = 0
    let dome_loop = vec![
        use_of(m, rim_edge, true, line((0.0, span.lo()), Vec2::x())),
        use_of(m, seam, true, line((turn.hi(), 0.0), Vec2::y())),
        use_of(m, apex, false, line((0.0, span.hi()), Vec2::x())),
        use_of(m, seam, false, line((turn.lo(), 0.0), Vec2::y())),
    ];
    // the base, its normal down: the rim walked clockwise about z
    let down = Frame::new(base.origin(), -Vec3::z(), Vec3::x()).map_err(OpError::from)?;
    let floor = Surface::Plane { frame: down };
    let rim_on_floor =
        pcurve_on(&rim_curve, turn, &floor, tol, &mut Meter::new(&Control::NONE)).map_err(geom)?;
    let floor_loop = vec![use_of(m, rim_edge, false, rim_on_floor)];
    let faces = vec![
        FaceSpec::New {
            surface: m.add_surface(Surface::Nurbs(dome)),
            orientation: Orientation::Forward,
            loops: vec![dome_loop],
            tolerance,
        },
        FaceSpec::New {
            surface: m.add_surface(floor),
            orientation: Orientation::Forward,
            loops: vec![floor_loop],
            tolerance,
        },
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
    let (solid, _) = ops::build(m, builder, &keys, &Control::NONE)?;
    Ok(solid)
}

/// Every slot keyed by its place in the assembly.
fn keyed<S: Copy + Ord>(slots: &[S]) -> BTreeMap<S, u64> {
    slots.iter().enumerate().map(|(i, &s)| (s, i as u64)).collect()
}

pub fn build(m: &mut Model) -> Result<Body, Refusal> {
    let bezier = vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0];
    let points = CONTROL.iter().map(|&[x, y, z]| Point3::new(x, y, z)).collect();
    let curve = NurbsCurve::new(3, bezier, points, vec![1.0; 4]).map_err(|e| e.to_string())?;
    turned(m, curve)
}
