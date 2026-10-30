//! A spheroid: the region in the xz plane between the z axis from (0, 0, −10) to (0, 0, 10) and
//! the rational quadratic B-spline through `CONTROL` with weights `WEIGHTS` on the knots `KNOTS`,
//! turned a full turn about z.
//!
//! arris's profiles hold lines and circular and elliptic arcs, so `revolve` cannot take the
//! spline. arris does turn a NURBS curve into its exact surface of revolution,
//! `NurbsSurface::revolution`, and leaves a consumer to build the face on it: the one face, with
//! a pole at each end of the axis, is assembled by `Builder::assemble` and finished by
//! `ops::build`, which proves the body with arris's checker.
use crate::Refusal;
use arris::geom::{Curve, Curve2, NurbsCurve, NurbsSurface, Surface};
use arris::math::{Control, Interval, Point2, Point3, UnitVec2, Vec2, Vec3};
use arris::ops::{self, BuildKeys, OpError};
use arris::topo::builder::{
    Assembly, Builder, EdgeKey, EdgeSpec, FaceSpec, UseSpec, VertexKey, VertexSpec,
};
use arris::topo::entity::EdgeGeometry;
use arris::topo::{Body, Model, Orientation};
use std::collections::BTreeMap;
use std::f64::consts::FRAC_1_SQRT_2;

const CONTROL: [[f64; 3]; 5] =
    [[0.0, 0.0, -10.0], [6.0, 0.0, -10.0], [6.0, 0.0, 0.0], [6.0, 0.0, 10.0], [0.0, 0.0, 10.0]];
const WEIGHTS: [f64; 5] = [1.0, FRAC_1_SQRT_2, 1.0, FRAC_1_SQRT_2, 1.0];
const KNOTS: [f64; 8] = [0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0];

// workaround: no NURBS curve in a profile
pub fn build(m: &mut Model) -> Result<Body, Refusal> {
    let geom = |e: arris::geom::GeomError| Refusal::from(e.to_string());
    let tolerance = m.precision().default_tolerance;
    let points = CONTROL.iter().map(|&[x, y, z]| Point3::new(x, y, z)).collect();
    let meridian =
        NurbsCurve::new(2, KNOTS.to_vec(), points, WEIGHTS.to_vec()).map_err(geom)?;
    let span = meridian.domain();
    let (south, north) = (meridian.eval(span.lo()).point, meridian.eval(span.hi()).point);
    // u the angle about z from the xz plane, v the meridian's parameter, a pole (a collapsed row)
    // at each end of it and the meridian itself the seam, at u = 0 and again at u = 2π
    let surface =
        NurbsSurface::revolution(&meridian, Point3::origin(), Vec3::z_axis(), Interval::TURN)
            .map_err(geom)?;
    let [turn, _] = surface.domain();

    let vertices = vec![
        VertexSpec::New { point: south, tolerance },
        VertexSpec::New { point: north, tolerance },
    ];
    let (s, n) = (0, 1);
    let edges = vec![
        EdgeSpec::New {
            geometry: EdgeGeometry::Curve {
                curve: m.add_curve(Curve::Nurbs(meridian)),
                range: span,
            },
            start: VertexKey::New(s),
            end: VertexKey::New(n),
            tolerance,
        },
        EdgeSpec::New {
            geometry: EdgeGeometry::Degenerate { range: turn },
            start: VertexKey::New(s),
            end: VertexKey::New(s),
            tolerance,
        },
        EdgeSpec::New {
            geometry: EdgeGeometry::Degenerate { range: turn },
            start: VertexKey::New(n),
            end: VertexKey::New(n),
            tolerance,
        },
    ];
    let (seam, south_pole, north_pole) = (0, 1, 2);
    let line = |m: &mut Model, (u, v): (f64, f64), along: Vec2| {
        m.add_curve2(Curve2::Line {
            origin: Point2::new(u, v),
            direction: UnitVec2::new_normalize(along),
        })
    };
    let use_of = |edge: usize, along: bool, pcurve| UseSpec {
        edge: EdgeKey::New(edge),
        orientation: if along { Orientation::Forward } else { Orientation::Reversed },
        pcurve,
    };
    // the face, its normal ∂u × ∂v outward: along the south pole, up the seam at u = 2π, back
    // along the north pole, down the seam at u = 0
    let walk = vec![
        use_of(south_pole, true, line(m, (0.0, span.lo()), Vec2::x())),
        use_of(seam, true, line(m, (turn.hi(), 0.0), Vec2::y())),
        use_of(north_pole, false, line(m, (0.0, span.hi()), Vec2::x())),
        use_of(seam, false, line(m, (turn.lo(), 0.0), Vec2::y())),
    ];
    let face = FaceSpec::New {
        surface: m.add_surface(Surface::Nurbs(surface)),
        orientation: Orientation::Forward,
        loops: vec![walk],
        tolerance,
    };
    let (builder, slots) =
        Builder::assemble(m, tolerance, Assembly { vertices, edges, shells: vec![vec![face]] })
            .map_err(OpError::from)?;
    let keys = BuildKeys {
        namespace: 0,
        vertices: keyed(&slots.vertices),
        edges: keyed(&slots.edges),
        faces: keyed(&slots.faces[0]),
        shells: vec![0],
        body: 0,
    };
    let (spheroid, _) = ops::build(m, builder, &keys, &Control::NONE)?;
    Ok(spheroid)
}

/// Every slot keyed by its place in the assembly.
fn keyed<S: Copy + Ord>(slots: &[S]) -> BTreeMap<S, u64> {
    slots.iter().enumerate().map(|(i, &s)| (s, i as u64)).collect()
}
