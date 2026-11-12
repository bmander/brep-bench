//! What the cadrs implementations of the chamfer cases share: the Chamfer feature on picked
//! edges, and the octahedron.
use crate::bench::{cut, extrude, p3, union, P3, V3};
use crate::Refusal;
use cadrs_kernel::{
    BodyId, BodyKind, ChamferMeasure, ChamferOpts, ChamferSpec, DefaultKernel, EdgeId, Kernel, LoftEnd,
    LoftSection, LoftSpec, Loop, Pick, Plane, Profile,
};

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
/// feature, each a chord from the edge in the plane square to it: `on_ball` on its face on the
/// ball about `centre`, through `ball_point`, and `on_other` on its other face, itself on a ball of
/// radius `other_ball` where that is given (else a plane or a cylinder along the section).
///
/// The distances are measured along the faces' tangent planes (`ChamferMeasure::Tangent`): on a
/// ball of radius R a tangent distance t lands at the angle atan(t/R) round it from the edge, so
/// the chord c is t = R·tan(2·asin(c/2R)); on a face straight across the edge the two are the
/// same. cadrs puts the first distance on the face whose loop runs along the edge: the face on
/// the left of the edge's direction, seen from outside the face (its outward normal on a ball
/// pointing away from the ball's centre).
#[allow(clippy::too_many_arguments)]
pub fn chord_two_distances_on_ball(
    k: &mut DefaultKernel,
    body: BodyId,
    edge_point: P3,
    centre: P3,
    ball_point: P3,
    on_ball: f64,
    on_other: f64,
    other_ball: Option<f64>,
) -> Result<BodyId, Refusal> {
    let tangent = |chord: f64, radius: f64| radius * (2.0 * (chord / (2.0 * radius)).asin()).tan();
    let radius = (edge_point - centre).norm();
    let on_ball = tangent(on_ball, radius);
    let on_other = other_ball.map_or(on_other, |r| tangent(on_other, r));
    let edge = k.edge_at(body, edge_point)?;
    let all = k.edges(body)?;
    let info = all.iter().find(|i| i.id == edge).ok_or("the edge is gone")?;
    let circle = info.circle.ok_or("the edge is not a circle")?;
    let axis = circle.normal.into_inner();
    let outward = edge_point - centre;
    // Whether the ball's face is the first of an edge's two: the face on the left of the edge's
    // direction, seen from outside. The direction where the edge starts: round the circle's axis
    // the way it runs.
    let ball_first = |i: &cadrs_kernel::EdgeInfo| {
        let sense = axis.cross(&(i.start - circle.center)).dot(&i.start_tangent).signum();
        let along = axis.cross(&(edge_point - circle.center)) * sense;
        outward.cross(&along).dot(&(ball_point - edge_point)) > 0.0
    };
    let first = ball_first(info);
    let spec = if first {
        ChamferSpec::TwoDistances(on_ball, on_other)
    } else {
        ChamferSpec::TwoDistances(on_other, on_ball)
    };
    // The whole circle: where the ball's seam and the other face's cross it apart, it is two
    // edges, each chamfered, one that runs the other way round with its distances flipped.
    let on_circle = |i: &&cadrs_kernel::EdgeInfo| {
        i.circle.is_some_and(|c| {
            (c.center - circle.center).norm() < 1e-9
                && (c.radius - circle.radius).abs() < 1e-9
                && c.normal.into_inner().cross(&axis).norm() < 1e-9
        })
    };
    let rim: Vec<&cadrs_kernel::EdgeInfo> = all.iter().filter(on_circle).collect();
    let mut opts = ChamferOpts::new(spec);
    opts.measurement = ChamferMeasure::Tangent;
    opts.flipped = rim.iter().filter(|i| ball_first(i) != first).map(|i| i.id).collect();
    let edges: Vec<EdgeId> = rim.iter().map(|i| i.id).collect();
    chamfer_with(k, body, &edges, &opts)
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
