//! A block from z = 0 to z = 10 whose plan, 80 along x and 40 along y with its corners rounded to
//! 5, has a lobe hanging from its south side: one closed chain of thirteen pieces, each running on
//! tangent into the next — the south side dropping 5 into the lobe by a concave quarter circle,
//! the lobe the lower half of the circle of radius 10 about (45, −5), a straight rise and a
//! reverse (concave) quarter circle up onto a step 5 above where the side began. The rim of its
//! top face, all thirteen edges, rounded to radius 2 as one round that never ends.
use crate::sketch::named;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{prism, Profile, ProfileEdge};
use cad_kernel::fillet::roll;
use cad_kernel::geom::{tolerance, Frame, V};
use cad_kernel::topo::Brep;

const HEIGHT: f64 = 10.;
const ROUND: f64 = 2.;

/// A piece of the plan: a straight side between two points, or an arc of the circle about a
/// centre of a radius, spanning two angles (degrees about +z from +x, the lesser first).
enum Piece {
    Line([f64; 2], [f64; 2]),
    Arc([f64; 2], f64, [f64; 2]),
}
use Piece::{Arc, Line};

/// The plan, anticlockwise from the south-west.
const PLAN: [Piece; 13] = [
    Line([5., 0.], [30., 0.]),            // south side
    Arc([30., -5.], 5., [0., 90.]),       // dip, concave
    Arc([45., -5.], 10., [180., 360.]),   // lobe
    Line([55., -5.], [55., 0.]),          // rise
    Arc([60., 0.], 5., [90., 180.]),      // reverse arc, concave
    Line([60., 5.], [75., 5.]),           // step
    Arc([75., 10.], 5., [-90., 0.]),      // corner
    Line([80., 10.], [80., 35.]),         // east side
    Arc([75., 35.], 5., [0., 90.]),       // corner
    Line([75., 40.], [5., 40.]),          // north side
    Arc([5., 35.], 5., [90., 180.]),      // corner
    Line([0., 35.], [0., 5.]),            // west side
    Arc([5., 5.], 5., [180., 270.]),      // corner
];

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
    // the plan as a profile, and each piece's middle on the top's rim
    let mut edges = Vec::new();
    let mut rim = Vec::new();
    for piece in &PLAN {
        match *piece {
            Line(a, b) => {
                edges.push(ProfileEdge::Line { a: [a[0], a[1], 0.], b: [b[0], b[1], 0.] });
                rim.push([0.5 * (a[0] + b[0]), 0.5 * (a[1] + b[1]), HEIGHT]);
            }
            Arc(c, r, span) => {
                let frame = Frame::new([c[0], c[1], 0.], [0., 0., 1.], [1., 0., 0.]);
                edges.push(ProfileEdge::Arc { frame, r, span: Some(span.map(f64::to_radians)) });
                let mid = (0.5 * (span[0] + span[1])).to_radians();
                rim.push([c[0] + r * mid.cos(), c[1] + r * mid.sin(), HEIGHT]);
            }
        }
    }
    let plan = Profile { origin: [0.; 3], normal: [0., 0., 1.], loops: vec![edges], names: vec![] };
    let block = prism(&plan, 0., HEIGHT)?;
    // the top's rim, each edge picked by its middle in order round it; fillet::roll rolls the ball
    // along the run between two faces, one edge here, so each is rolled in turn
    rim_rounded(&block, &rim, ROUND)
}
