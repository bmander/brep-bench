//! A block on the square 0 ≤ x, y ≤ 20, its floor at z = 0, under the bicubic Bézier patch whose
//! sixteen control points stand over an even 4 × 4 grid of the square at the heights `HEIGHTS`.
//!
//! arris has no operation that makes a solid from a NURBS surface, so the block is built as arris
//! leaves a consumer to build topology of its own: the patch is an arris `NurbsSurface`, its four
//! edges the isocurves it ends in, and the six faces are assembled by `Builder::assemble` and
//! finished by `ops::build`, which proves the body with arris's checker.
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

pub const SIDE: f64 = 20.0;
/// zᵢⱼ: row i at x = 20i/3, column j at y = 20j/3.
const HEIGHTS: [[f64; 4]; 4] = [
    [4.0, 6.0, 7.0, 5.0],
    [6.0, 14.0, 12.0, 7.0],
    [5.0, 13.0, 16.0, 8.0],
    [3.0, 6.0, 8.0, 6.0],
];

/// The block standing on the floor z = 0 under `top`, a NURBS surface clamped in both directions
/// whose `u` runs along +x and `v` along +y, so that its normal is up and its four edges, the
/// isocurves at the ends of its domain, lie in the planes of the four upright sides.
pub fn block_under(m: &mut Model, top: NurbsSurface) -> Result<Body, Refusal> {
    let geom = |e: GeomError| Refusal::from(e.to_string());
    let tol = m.precision().tolerance();
    let tolerance = m.precision().default_tolerance;
    let [n, k] = top.counts();
    let [du, dv] = top.domain();
    // the corners a, b, c, d of the floor counter-clockwise from x and y least, and A … D above
    let corner = |u: f64, v: f64| top.eval(u, v).point;
    let up = [corner(du.lo(), dv.lo()), corner(du.hi(), dv.lo()), corner(du.hi(), dv.hi()),
        corner(du.lo(), dv.hi())];
    let points: Vec<Point3> =
        up.iter().map(|q| Point3::new(q.x, q.y, 0.0)).chain(up).collect();
    let (a, b, c, d, ta, tb, tc, td) = (0, 1, 2, 3, 4, 5, 6, 7);

    // the top's edges: at u's ends a control column; at v's ends a control row, the curve over
    // the u knots through the net's points in that row
    let row = |j: usize| {
        let net = (0..n).map(|i| top.control_point(i, j).expect("a point of the net")).collect();
        let weights = (0..n).map(|i| top.weights()[i * k + j]).collect();
        NurbsCurve::new(top.degree()[0], top.knots()[0].to_vec(), net, weights)
    };
    let line = |from: usize, to: usize| -> Result<(Curve, Interval), Refusal> {
        let along = points[to] - points[from];
        let curve = Curve::Line { origin: points[from], direction: UnitVec3::new_normalize(along) };
        Ok((curve, Interval::new(0.0, along.norm()).map_err(|e| e.to_string())?))
    };
    // each edge's curve, its range and its two ends: the floor's a→b, b→c, c→d, d→a, the
    // uprights a→A … d→D, then the top's A→B, B→C, D→C and A→D, each along its isocurve
    let mut edges: Vec<(Curve, Interval, usize, usize)> = Vec::new();
    for (from, to) in [(a, b), (b, c), (c, d), (d, a), (a, ta), (b, tb), (c, tc), (d, td)] {
        let (curve, range) = line(from, to)?;
        edges.push((curve, range, from, to));
    }
    edges.push((Curve::Nurbs(row(0).map_err(geom)?), du, ta, tb));
    edges.push((Curve::Nurbs(top.control_column(n - 1).map_err(geom)?), dv, tb, tc));
    edges.push((Curve::Nurbs(row(k - 1).map_err(geom)?), du, td, tc));
    edges.push((Curve::Nurbs(top.control_column(0).map_err(geom)?), dv, ta, td));
    let (ab, bc, cd, da, aa, bb, cc, dd) = (0, 1, 2, 3, 4, 5, 6, 7);
    let (tab, tbc, tdc, tad) = (8, 9, 10, 11);

    // the floor and the sides, each a plane framed with its normal outward, and the top
    let plane = |origin: Point3, outward: Vec3, x: Vec3| -> Result<Surface, OpError> {
        Ok(Surface::Plane { frame: Frame::new(origin, outward, x)? })
    };
    let (lo, hi) = (points[a], points[c]);
    let floor = plane(Point3::origin(), -Vec3::z(), Vec3::x())?;
    let front = plane(lo, -Vec3::y(), Vec3::x())?;
    let right = plane(hi, Vec3::x(), Vec3::y())?;
    let back = plane(hi, Vec3::y(), -Vec3::x())?;
    let left = plane(lo, -Vec3::x(), -Vec3::y())?;
    let dome = Surface::Nurbs(top);
    // on the top, each edge's pcurve is the line of its isocurve, at the curve's own parameter
    let iso = |(u, v): (f64, f64), along: Vec2| Curve2::Line {
        origin: Point2::new(u, v),
        direction: UnitVec2::new_normalize(along),
    };
    let on_top = BTreeMap::from([
        (tab, iso((0.0, dv.lo()), Vec2::x())),
        (tbc, iso((du.hi(), 0.0), Vec2::y())),
        (tdc, iso((0.0, dv.hi()), Vec2::x())),
        (tad, iso((du.lo(), 0.0), Vec2::y())),
    ]);
    // each face's loop, walked counter-clockwise seen from outside: (edge, along its curve)
    let faces: [(Surface, [(usize, bool); 4]); 6] = [
        (floor, [(da, false), (cd, false), (bc, false), (ab, false)]),
        (front, [(ab, true), (bb, true), (tab, false), (aa, false)]),
        (right, [(bc, true), (cc, true), (tbc, false), (bb, false)]),
        (back, [(cd, true), (dd, true), (tdc, true), (cc, false)]),
        (left, [(da, true), (aa, true), (tad, true), (dd, false)]),
        (dome, [(tab, true), (tbc, true), (tdc, false), (tad, false)]),
    ];

    let mut meter = Meter::new(&Control::NONE);
    let mut shell = Vec::new();
    for (surface, walk) in faces {
        let mut uses = Vec::new();
        for (e, along) in walk {
            let (curve, range, _, _) = &edges[e];
            let pcurve = match &surface {
                Surface::Nurbs(_) => on_top[&e].clone(),
                plane => pcurve_on(curve, *range, plane, tol, &mut meter).map_err(geom)?,
            };
            let orientation = if along { Orientation::Forward } else { Orientation::Reversed };
            uses.push(UseSpec { edge: EdgeKey::New(e), orientation, pcurve: m.add_curve2(pcurve) });
        }
        shell.push(FaceSpec::New {
            surface: m.add_surface(surface),
            orientation: Orientation::Forward,
            loops: vec![uses],
            tolerance,
        });
    }
    let assembly = Assembly {
        vertices: points.iter().map(|&point| VertexSpec::New { point, tolerance }).collect(),
        edges: edges
            .into_iter()
            .map(|(curve, range, from, to)| EdgeSpec::New {
                geometry: EdgeGeometry::Curve { curve: m.add_curve(curve), range },
                start: VertexKey::New(from),
                end: VertexKey::New(to),
                tolerance,
            })
            .collect(),
        shells: vec![shell],
    };
    let (builder, slots) = Builder::assemble(m, tolerance, assembly).map_err(OpError::from)?;
    let keys = BuildKeys {
        namespace: 0,
        vertices: keyed(&slots.vertices),
        edges: keyed(&slots.edges),
        faces: keyed(&slots.faces[0]),
        shells: vec![0],
        body: 0,
    };
    let (block, _) = ops::build(m, builder, &keys, &Control::NONE)?;
    Ok(block)
}

/// Every slot keyed by its place in the assembly.
fn keyed<S: Copy + Ord>(slots: &[S]) -> BTreeMap<S, u64> {
    slots.iter().enumerate().map(|(i, &s)| (s, i as u64)).collect()
}

/// The patch: Pᵢⱼ = (20i/3, 20j/3, zᵢⱼ), cubic both ways on the Bézier knots, every weight 1.
fn patch() -> Result<NurbsSurface, Refusal> {
    let bezier = vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0];
    let at = |i: usize| SIDE * i as f64 / 3.0;
    let net = (0..4)
        .flat_map(|i| (0..4).map(move |j| Point3::new(at(i), at(j), HEIGHTS[i][j])))
        .collect();
    NurbsSurface::new([3, 3], [bezier.clone(), bezier], net, vec![1.0; 16])
        .map_err(|e| e.to_string().into())
}

pub fn build(m: &mut Model) -> Result<Body, Refusal> {
    block_under(m, patch()?)
}
