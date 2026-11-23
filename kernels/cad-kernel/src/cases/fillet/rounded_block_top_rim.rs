//! A block from (−10, −6, 0) to (10, 6, 5), its vertical edges rounded to 3, the rim of its top
//! face rounded to 1: eight edges, straight and quarter circles, each running on tangent into the
//! next.
use crate::sketch::{named, rounded_rectangle};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::prism;
use cad_kernel::fillet::roll;
use cad_kernel::geom::{tolerance, V};
use cad_kernel::topo::Brep;
use std::f64::consts::FRAC_1_SQRT_2;

const HALF: [f64; 2] = [10., 6.];
const CORNER: f64 = 3.;
const HEIGHT: f64 = 5.;
const ROUND: f64 = 1.;

/// `part` with each edge through a point of `rim` rounded to `r`, the edges running on tangent into
/// one another round a loop: every round rolled against `part` as it stands, before any is applied,
/// then each piece filled in at a concave edge or taken off a convex one in turn — the pieces end
/// in the sections through the seams between the edges, where each meets the next. (`fillet::roll`
/// reads the band its ball sweeps round the whole spine, past the run's ends too: rolled after a
/// neighbour's round has been applied, the face's edge with that round lies in the band and the
/// face is refused as too short for the ball.) Every face is named apart first: a roll reads faces
/// of one name as pieces of one face — the prism's sides are all `` — and it refuses a run whose
/// end meets other pieces of its own faces as a mitre.
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
    let block = prism(&rounded_rectangle([-HALF[0], -HALF[1]], HALF, CORNER, 0.), 0., HEIGHT)?;
    // the rim's edges, each picked by its middle, in order round the top: fillet::roll rolls the
    // ball along the run between two faces, which here is one edge, so each is rolled in turn
    let [a, b] = HALF;
    let off = CORNER * (1. - FRAC_1_SQRT_2);
    let rim: [V; 8] = [
        [0., b, HEIGHT], [a - off, b - off, HEIGHT], [a, 0., HEIGHT], [a - off, -b + off, HEIGHT],
        [0., -b, HEIGHT], [-a + off, -b + off, HEIGHT], [-a, 0., HEIGHT], [-a + off, b - off, HEIGHT],
    ];
    rim_rounded(&block, &rim, ROUND)
}
