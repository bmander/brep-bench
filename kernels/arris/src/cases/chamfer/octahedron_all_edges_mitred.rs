//! The octahedron |x| + |y| + |z| ≤ 10 with all twelve of its edges chamfered √6 on both faces,
//! one edge after another, so the four chamfers at each corner run on and meet in a point, the
//! corner mitred.
use crate::solids::octahedron;
use crate::Refusal;
use arris::math::{Control, Point3};
use arris::ops::{chamfer, OpError};
use arris::topo::{Body, Edge, Model};

const SIZE: f64 = 10.0;
/// √6, across a face square to the edge: the chamfer of the edge from (10, 0, 0) to (0, 10, 0)
/// meets the face x + y + z = 10 on the line z = 2.
const DISTANCE: f64 = 2.449489742783178;
/// How far an end of an edge may lie from the line of the octahedron's edge it is what is left of.
const ON_LINE: f64 = 1e-7;

/// The octahedron's corner `k`: on axis `k / 2`, at +10 for an even `k` and −10 for an odd one.
fn corner(k: usize) -> Point3 {
    let mut p = Point3::origin();
    p[k / 2] = if k % 2 == 0 { SIZE } else { -SIZE };
    p
}

/// The edge of `body` on the line through `a` and `b`: what is left of the octahedron's edge
/// between those corners, its ends cut back by the chamfers made before it at either corner, not
/// necessarily as far at each.
fn edge_on(m: &Model, body: Body, a: Point3, b: Point3) -> Result<Edge, Refusal> {
    let along = (b - a).normalize();
    let off = |p: Point3| (p - a - along * (p - a).dot(&along)).norm();
    for edge in m.edges(body).map_err(OpError::from)? {
        let Some((curve, range)) = m.edge(edge.id).map_err(OpError::from)?.curve() else {
            continue;
        };
        let curve = m.curve(curve).map_err(OpError::from)?;
        if [range.lo(), range.hi()].into_iter().all(|t| off(curve.point(t)) < ON_LINE) {
            return Ok(edge);
        }
    }
    Err(format!("no edge left on the line from {a} to {b}").into())
}

pub fn build(m: &mut Model) -> Result<Body, Refusal> {
    let mut solid = octahedron(m, SIZE)?;
    // Each edge its own chamfer, between corners on different axes: a chamfer made alone ends on
    // the faces at its ends, the chamfers made before it among them, so the four at a corner
    // mitre. (All twelve in one `chamfer` is chamfer/octahedron-all-edges, each corner closed by
    // a square.) arris 0.9.1 chamfers the first edge, from (10, 0, 0) to
    // (0, 10, 0), each end cut by the fan of faces across it, and declines the second, the first
    // to end on an earlier chamfer at a corner ("the blend leaves its face through an edge that
    // is not the corner's own"), beside it or opposite it there. In any order, some edge is the
    // second at a corner: three edges at most share none.
    for a in 0..6 {
        for b in (a + 1..6).filter(|b| b / 2 != a / 2) {
            let edge = edge_on(m, solid, corner(a), corner(b))?;
            let (chamfered, _) = chamfer(m, solid, &[edge], DISTANCE, &Control::NONE)?;
            solid = chamfered;
        }
    }
    Ok(solid)
}
