//! What the cadrs implementations of the fillet cases share: the Fillet feature on picked edges,
//! the edges round a top face, a variable round of one edge, the half cylinder and the spline bean.
use crate::bench::{arc, extrude, p2, P2, P3};
use crate::shared::geometry::Section;
use crate::Refusal;
use cadrs_kernel::{BodyId, Curve2, DefaultKernel, EdgeId, FilletLaw, Kernel, Loop, Pick, Plane, Profile};

/// `body` with `edges` (and the edges tangent to them) rounded to `radius` in one Fillet
/// feature.
pub fn fillet(
    k: &mut DefaultKernel,
    body: BodyId,
    edges: &[EdgeId],
    radius: f64,
) -> Result<BodyId, Refusal> {
    let r = k.fillet(body, edges, radius)?;
    r.bodies.first().copied().ok_or_else(|| Refusal::from("the fillet made no body"))
}

/// `body` with each law's edge rounded with the radius it gives along the edge (a variable
/// fillet), overflow allowed.
pub fn fillet_variable(
    k: &mut DefaultKernel,
    body: BodyId,
    laws: &[FilletLaw],
) -> Result<BodyId, Refusal> {
    let r = k.fillet_variable(body, laws, true)?;
    r.bodies.first().copied().ok_or_else(|| Refusal::from("the fillet made no body"))
}

/// The edges of `body` lying in the plane at height `z` (their ends and middle there).
pub fn edges_at_height(k: &mut DefaultKernel, body: BodyId, z: f64) -> Result<Vec<EdgeId>, Refusal> {
    let near = |p: &P3| (p.z - z).abs() < 1e-7;
    let edges = k.edges_where(body, &|e| near(&e.start) && near(&e.mid) && near(&e.end))?;
    if edges.is_empty() {
        return Err(Refusal::from(format!("no edge at z = {z}")));
    }
    Ok(edges)
}

/// `body` with `edge` rounded by a radius running linearly along it, `from` at the end where
/// `starts_here` holds and `to` at the other (a variable fillet).
pub fn fillet_linear(
    k: &mut DefaultKernel,
    body: BodyId,
    edge: EdgeId,
    starts_here: &dyn Fn(&P3) -> bool,
    from: f64,
    to: f64,
) -> Result<BodyId, Refusal> {
    let info = k.edges(body)?.into_iter().find(|e| e.id == edge).ok_or("the edge is gone")?;
    // the law runs from the edge's start to its end
    let radii = if starts_here(&info.start) { vec![(0.0, from), (1.0, to)] } else { vec![(0.0, to), (1.0, from)] };
    fillet_variable(k, body, &[FilletLaw { edge, radii }])
}

/// The half of the cylinder of `radius` on the z axis from z = 0 to `height` on the side x ≥ 0:
/// the half disk extruded.
pub fn half_cylinder(k: &mut DefaultKernel, radius: f64, height: f64) -> Result<BodyId, Refusal> {
    let half = Loop::new(vec![
        arc(p2(0.0, 0.0), radius, -90.0, 180.0),
        Curve2::line([0.0, radius], [0.0, -radius]),
    ]);
    extrude(k, &Profile::of(Plane::xy(0.0), vec![half]), height)
}

/// The bean of the spline rim cases: the periodic uniform cubic B-spline on these eight control
/// points.
pub const BEAN: [(f64, f64); 8] =
    [(-10.0, -5.0), (0.0, -7.0), (10.0, -5.0), (12.0, 4.0), (4.0, 7.0), (0.0, 3.0), (-4.0, 7.0), (-12.0, 4.0)];

/// The prism from z = 0 to `height` on the bean.
pub fn bean(k: &mut DefaultKernel, height: f64) -> Result<BodyId, Refusal> {
    Section::on(Plane::xy(0.0)).periodic(3, &BEAN).slid(k, crate::bench::v3(0.0, 0.0, height))
}

/// The closed outline through `corners` (in order, either way round) with corner k rounded to
/// `radii[k]` (0 leaves it sharp): each corner's arc tangent to the two sides there, turning
/// the way they turn, so a re-entrant corner's arc bulges into the region. Each side keeps the
/// straight stretch between its two corners' arcs.
pub fn rounded_polygon(corners: &[(f64, f64)], radii: &[f64]) -> Vec<Curve2> {
    let n = corners.len();
    let at = |k: usize| corners[k % n];
    let dir = |k: usize| {
        let ((ax, ay), (bx, by)) = (at(k), at(k + 1));
        let len = (bx - ax).hypot(by - ay);
        ((bx - ax) / len, (by - ay) / len)
    };
    // Where corner k's arc leaves the side before it and joins the side after, its centre and
    // its turn (radians, anticlockwise positive).
    let rounds: Vec<((f64, f64), (f64, f64), (f64, f64), f64)> = (0..n)
        .map(|k| {
            let (c, a, b) = (at(k), dir(k + n - 1), dir(k));
            let turn = (a.0 * b.1 - a.1 * b.0).atan2(a.0 * b.0 + a.1 * b.1);
            let r = radii[k];
            let t = r * (turn.abs() / 2.0).tan();
            let leave = (c.0 - a.0 * t, c.1 - a.1 * t);
            let join = (c.0 + b.0 * t, c.1 + b.1 * t);
            let side = turn.signum();
            let centre = (leave.0 - a.1 * r * side, leave.1 + a.0 * r * side);
            (leave, join, centre, turn)
        })
        .collect();
    let p = |q: (f64, f64)| -> P2 { p2(q.0, q.1) };
    let mut path = Vec::new();
    for k in 0..n {
        let (leave, join, centre, turn) = rounds[k];
        if radii[k] > 0.0 {
            let start = (leave.1 - centre.1).atan2(leave.0 - centre.0).to_degrees();
            path.push(arc(p(centre), radii[k], start, turn.to_degrees()));
        }
        let next = rounds[(k + 1) % n].0;
        path.push(Curve2::line(p(join), p(next)));
    }
    path
}

