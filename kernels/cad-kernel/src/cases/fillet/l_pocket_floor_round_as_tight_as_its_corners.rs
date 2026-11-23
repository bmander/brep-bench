//! A block from (−20, −20, 0) to (20, 20, 8) with an L-shaped pocket cut 5 deep into its top: the
//! polygon through (−12, −12), (12, −12), (12, 0), (0, 0), (0, 12) and (−12, 12), its five convex
//! corners rounded to radius 1 and its re-entrant corner at the origin to radius 3, about (3, 3).
//! The rim of the pocket's floor, twelve edges each running on tangent into the next, rounded to
//! radius 1, filling: as tight as the convex corners, so round each of them the round is an eighth
//! of a ball, and round the re-entrant corner a quarter of a torus's tube about the circle of
//! radius 4 about (3, 3). The block's top rim and the pocket's top rim are left sharp.
use crate::sketch::{block, named};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{prism, Profile, ProfileEdge};
use cad_kernel::fillet::roll;
use cad_kernel::geom::{tolerance, Frame, V};
use cad_kernel::topo::Brep;

const BLOCK: [V; 2] = [[-20., -20., 0.], [20., 20., 8.]];
const DEPTH: f64 = 5.;
const CONVEX: f64 = 1.;
const REENTRANT: f64 = 3.;
const ROUND: f64 = 1.;

/// The pocket's rounded corners in order round it: each arc's centre, radius and the angles (in
/// degrees, about +z from +x) it spans.
const CORNERS: [([f64; 2], f64, [f64; 2]); 6] = [
    ([11., -11.], CONVEX, [-90., 0.]),
    ([11., -1.], CONVEX, [0., 90.]),
    ([3., 3.], REENTRANT, [180., 270.]),
    ([-1., 11.], CONVEX, [0., 90.]),
    ([-11., 11.], CONVEX, [90., 180.]),
    ([-11., -11.], CONVEX, [180., 270.]),
];

/// The point at angle `deg` on the circle of radius `r` about `c`, at height `z`.
fn on(c: [f64; 2], r: f64, deg: f64, z: f64) -> V {
    let a = deg.to_radians();
    [c[0] + r * a.cos(), c[1] + r * a.sin(), z]
}

/// `part` with each edge through a point of `rim` rounded to `r`, the edges running on tangent into
/// one another round a loop: every round rolled against `part` as it stands, before any is applied,
/// then each piece filled in at a concave edge or taken off a convex one in turn — the pieces end
/// in the sections through the seams between the edges, where each meets the next. (`fillet::roll`
/// reads the band its ball sweeps round the whole spine, past the run's ends too: rolled after a
/// neighbour's round has been applied, the face's edge with that round lies in the band and the
/// face is refused as too short for the ball.) Every face is named apart first: a roll reads faces
/// of one name as pieces of one face, and the faces the Booleans leave share theirs.
fn rim_rounded(part: &Brep, rim: &[V], r: f64) -> Result<Brep, String> {
    let part = named(part.clone());
    let tol = tolerance(part.size());
    let rounds = rim.iter()
        .map(|&at| roll(&part, at, r, tol).map_err(|e| format!("the edge at {at:?}: {e}")))
        .collect::<Result<Vec<_>, String>>()?;
    rounds.iter().zip(rim).try_fold(part.clone(), |p, (round, at)| {
        let op = if round.concave { Op::Union } else { Op::Cut };
        combined(&p, &round.piece, op, 0.).map_err(|e| format!("the round at {at:?} applied: {e}"))
    })
}

pub fn build() -> Result<Brep, String> {
    let floor = BLOCK[1][2] - DEPTH;
    // the pocket's outline on its floor: each corner's arc, and the straight side from its end to
    // the next corner's start (at the re-entrant corner the arc runs the other way round)
    let ends = |k: usize| {
        let (c, r, [a, b]) = CORNERS[k];
        let (start, end) = if r == REENTRANT { (b, a) } else { (a, b) };
        (on(c, r, start, floor), on(c, r, end, floor))
    };
    let mut edges = Vec::new();
    // and the rim's edges, each picked by its middle, in order round it
    let mut rim = Vec::new();
    for k in 0..CORNERS.len() {
        let (c, r, span) = CORNERS[k];
        let frame = Frame::new([c[0], c[1], floor], [0., 0., 1.], [1., 0., 0.]);
        edges.push(ProfileEdge::Arc { frame, r, span: Some(span.map(f64::to_radians)) });
        rim.push(on(c, r, 0.5 * (span[0] + span[1]), floor));
        let (a, b) = (ends(k).1, ends((k + 1) % CORNERS.len()).0);
        edges.push(ProfileEdge::Line { a, b });
        rim.push([0.5 * (a[0] + b[0]), 0.5 * (a[1] + b[1]), floor]);
    }
    let (origin, normal) = ([0., 0., floor], [0., 0., 1.]);
    let outline = Profile { origin, normal, loops: vec![edges], names: vec![] };
    let pocket = prism(&outline, 0., DEPTH + 1.)?;
    let part = combined(&block(BLOCK[0], BLOCK[1])?, &pocket, Op::Cut, 0.)?;
    // fillet::roll rolls the ball along the run between two faces, one edge here, so each is
    // rolled in turn. Round each convex corner the ball is as wide as the corner's arc and stands
    // still, an eighth of it the corner's round, and the runs either side end in the section it
    // stands in; round the re-entrant corner it turns about the arc's axis, a torus's tube.
    rim_rounded(&part, &rim, ROUND)
}
