//! A block from (−40, −30, 0) to (40, 30, 16) with a pocket cut 10 deep into its top: the
//! rectangle from (−25, −15) to (25, 15) with its corners rounded to radius 4. The rim of the
//! pocket's floor, eight edges each running on tangent into the next, rounded to radius 4,
//! filling: the corners' own radius, so round each corner the round is an eighth of the ball of
//! radius 4 about (±21, ±11, 10). The block's top rim and the pocket's top rim are left sharp.
use crate::sketch::{block, named, rounded_rectangle};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::prism;
use cad_kernel::fillet::roll;
use cad_kernel::geom::{tolerance, V};
use cad_kernel::topo::Brep;
use std::f64::consts::FRAC_1_SQRT_2;

const BLOCK: [V; 2] = [[-40., -30., 0.], [40., 30., 16.]];
const POCKET: [f64; 2] = [25., 15.];
const CORNER: f64 = 4.;
const DEPTH: f64 = 10.;
const ROUND: f64 = 4.;

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
    let top = BLOCK[1][2];
    let floor = top - DEPTH;
    let [a, b] = POCKET;
    // the pocket: its rounded outline on the floor swept up past the top
    let pocket = prism(&rounded_rectangle([-a, -b], [a, b], CORNER, floor), 0., DEPTH + 1.)?;
    let part = combined(&block(BLOCK[0], BLOCK[1])?, &pocket, Op::Cut, 0.)?;
    // the floor's rim, eight edges each picked by its middle in order round it: the sides' at
    // their middles, the quarter circles' halfway round. fillet::roll rolls the ball along the run
    // between two faces, one edge here, so each is rolled in turn. Round each corner the ball is as
    // wide as the corner's arc and stands still, an eighth of it the corner's round, and the
    // straight runs either side end in the section it stands in.
    let off = CORNER * (1. - FRAC_1_SQRT_2);
    let rim: [V; 8] = [
        [0., -b, floor], [a - off, -b + off, floor], [a, 0., floor], [a - off, b - off, floor],
        [0., b, floor], [-a + off, b - off, floor], [-a, 0., floor], [-a + off, -b + off, floor],
    ];
    rim_rounded(&part, &rim, ROUND)
}
