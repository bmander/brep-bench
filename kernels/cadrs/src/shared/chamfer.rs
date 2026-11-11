//! What the cadrs implementations of the chamfer cases share: the Chamfer feature on picked
//! edges, and the octahedron.
use crate::bench::{cut, extrude, p3, union, P3, V3};
use crate::Refusal;
use cadrs_kernel::{BodyId, BodyKind, ChamferOpts, ChamferSpec, DefaultKernel, EdgeId, Kernel, LoftEnd, LoftSection, LoftSpec, Loop, Pick, Plane, Profile};

/// `body` with `edges` chamfered by `spec` in one Chamfer feature.
pub fn chamfer(
    k: &mut DefaultKernel,
    body: BodyId,
    edges: &[EdgeId],
    spec: ChamferSpec,
) -> Result<BodyId, Refusal> {
    chamfer_with(k, body, edges, &ChamferOpts::new(spec))
}

/// `body` with `edges` chamfered with the Chamfer dialog's options.
pub fn chamfer_with(
    k: &mut DefaultKernel,
    body: BodyId,
    edges: &[EdgeId],
    opts: &ChamferOpts,
) -> Result<BodyId, Refusal> {
    let r = k.chamfer_with(body, edges, opts)?;
    r.bodies.first().copied().ok_or_else(|| Refusal::from("the chamfer made no body"))
}

/// `body` with the circular edge through `edge_point` chamfered by two distances in one Chamfer
/// feature: `on_ball` on its face on the ball about `centre`, through `ball_point`, and
/// `on_other` on its other face. cadrs puts the first distance on the face whose loop runs along
/// the edge: the face on the left of the edge's direction, seen from outside the face (its
/// outward normal on a ball pointing away from the ball's centre).
pub fn two_distances_on_ball(
    k: &mut DefaultKernel,
    body: BodyId,
    edge_point: P3,
    centre: P3,
    ball_point: P3,
    on_ball: f64,
    on_other: f64,
) -> Result<BodyId, Refusal> {
    let edge = k.edge_at(body, edge_point)?;
    let info = k.edges(body)?.into_iter().find(|i| i.id == edge).ok_or("the edge is gone")?;
    let circle = info.circle.ok_or("the edge is not a circle")?;
    let axis = circle.normal.into_inner();
    // The edge's direction at the point picked: round the circle's axis the way it runs.
    let sense = axis.cross(&(info.start - circle.center)).dot(&info.start_tangent).signum();
    let along = axis.cross(&(edge_point - circle.center)) * sense;
    let outward = edge_point - centre;
    let ball_first = outward.cross(&along).dot(&(ball_point - edge_point)) > 0.0;
    let spec = if ball_first {
        ChamferSpec::TwoDistances(on_ball, on_other)
    } else {
        ChamferSpec::TwoDistances(on_other, on_ball)
    };
    chamfer(k, body, &[edge], spec)
}

/// The octahedron |x| + |y| + |z| ≤ `a`: the square through its four corners in the plane z = 0
/// lofted to a point at each of its other two corners, (0, 0, a) and (0, 0, −a), the two
/// pyramids united.
pub fn octahedron(k: &mut DefaultKernel, a: f64) -> Result<BodyId, Refusal> {
    let mut pyramid = |tip: f64| -> Result<BodyId, Refusal> {
        let square = Profile::of(Plane::xy(0.0), vec![Loop::polygon(&[[a, 0.0], [0.0, a], [-a, 0.0], [0.0, -a]])]);
        let spec = LoftSpec {
            body: BodyKind::Solid,
            sections: vec![LoftSection::Profile(square), LoftSection::Point(p3(0.0, 0.0, tip))],
            start: LoftEnd::default(),
            end: LoftEnd::default(),
            source: 1,
        };
        let r = k.loft_with(&spec)?;
        r.bodies.first().copied().ok_or_else(|| Refusal::from("the loft made no body"))
    };
    let upper = pyramid(a)?;
    let lower = pyramid(-a)?;
    union(k, upper, lower)
}
