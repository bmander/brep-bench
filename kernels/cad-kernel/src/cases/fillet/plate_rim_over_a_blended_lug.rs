//! A plate from (−40, −25, 0) to (40, 25, 13), its four upright edges rounded to radius 10, joined
//! to a lug, the cylinder of radius 9 about the upright through (0, 25) from z = 0 to 13. The two
//! concave upright edges where the lug's side meets the plate's north side, at (±9, 25, z),
//! rounded to radius 8; then the rim of the plate's bottom so blended, all twelve edges where the
//! bottom meets a side, rounded to radius 2 as one round that never ends.
//!
//! The plate, the lug and their blends are all upright, so they are drawn in the plan and swept
//! up: the plate's rounded rectangle joined to the lug's disc (`planar::boolean`), the two corners
//! where the north side meets the lug's circle rounded to 8 there (`corners::round`), and the
//! region swept 13 up. Each blend is then a cylinder of radius 8, exactly. (Rolled in space
//! instead, `fillet::roll`'s blend between the side's plane and the lug's cylinder is a B-spline
//! canal fitted to it, and the rim's rounds along it are rolled against that fit: each of them
//! touches its neighbour's at a point of their shared section rather than along it, and the
//! Boolean taking them off refuses the touch.)
use crate::sketch::named;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::prism;
use cad_kernel::corners::round;
use cad_kernel::fillet::roll;
use cad_kernel::geom::{tolerance, V};
use cad_kernel::planar::{self, Region, Seg};
use cad_kernel::recipe::region_profile;
use cad_kernel::topo::Brep;
use std::f64::consts::{FRAC_1_SQRT_2, FRAC_PI_2, PI};

const HALF: [f64; 2] = [40., 25.];
const HEIGHT: f64 = 13.;
const CORNER: f64 = 10.;
const LUG: f64 = 9.;
const BLEND: f64 = 8.;
const RIM: f64 = 2.;

/// The plan's steps, tagged: the north side, the lug's circle and the blends between them.
const NORTH: u32 = 1;
const LUG_SIDE: u32 = 2;
const BLENDS: u32 = 3;

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
    let [a, b] = HALF;
    let tol = tolerance(2. * a);
    // the plate's plan: its rounded rectangle, anticlockwise, the north side tagged
    let (ia, ib) = (a - CORNER, b - CORNER);
    let arc = |c: [f64; 2], a0: f64| Seg::Arc { c, r: CORNER, a0, sweep: FRAC_PI_2 };
    let plate = Region::plain(vec![vec![
        Seg::Line { a: [-ia, -b], b: [ia, -b] }, arc([ia, -ib], -FRAC_PI_2),
        Seg::Line { a: [a, -ib], b: [a, ib] }, arc([ia, ib], 0.),
        Seg::Line { a: [ia, b], b: [-ia, b] }, arc([-ia, ib], FRAC_PI_2),
        Seg::Line { a: [-a, ib], b: [-a, -ib] }, arc([-ia, -ib], PI),
    ]]).tagged(&|_, k| if k == 4 { NORTH } else { 0 });
    // the lug's disc, its circle in two halves
    let half = |a0: f64| Seg::Arc { c: [0., b], r: LUG, a0, sweep: PI };
    let lug = Region::plain(vec![vec![half(0.), half(PI)]]).tagged(&|_, _| LUG_SIDE);
    let plan = planar::boolean(&plate, &lug, planar::Op::Union, tol)?;
    // the blends: the two corners where the north side meets the lug's circle, rounded to BLEND
    let plan = round(&plan, &[NORTH], &[LUG_SIDE], BLEND, BLENDS)?;
    let name = |t: u32| format!("step {t}");
    let outline = region_profile(&plan, [0.; 3], [1., 0., 0.], [0., 1., 0.], &name);
    let part = prism(&outline, 0., HEIGHT)?;
    // the bottom's rim, twelve edges each picked by a point on it in order round it: the sides',
    // the corners' halfway round, the north side's two pieces (from the blends to the corners) at
    // their middles, each blend halfway between its touches, and the lug's arc at its northmost
    // point. A blend's axis stands BLEND off the side and LUG + BLEND off the lug's axis.
    let off = CORNER * (1. - FRAC_1_SQRT_2);
    let x = ((LUG + BLEND).powi(2) - BLEND * BLEND).sqrt();
    let north = 0.5 * (x + ia);
    let blend_at = |s: f64| {
        // the blend's axis, and the ways from it to its touches on the side (−y) and the lug
        let (cx, cy) = (s * x, b + BLEND);
        let l = (cx * cx + BLEND * BLEND).sqrt();
        let (t, u) = (-cx / l, -BLEND / l - 1.);
        let m = (t * t + u * u).sqrt();
        [cx + BLEND * t / m, cy + BLEND * u / m, 0.]
    };
    let rim: Vec<V> = vec![
        [0., -b, 0.], [a - off, -b + off, 0.], [a, 0., 0.], [a - off, b - off, 0.],
        [north, b, 0.], blend_at(1.), [0., b + LUG, 0.], blend_at(-1.), [-north, b, 0.],
        [-a + off, b - off, 0.], [-a, 0., 0.], [-a + off, -b + off, 0.],
    ];
    rim_rounded(&part, &rim, RIM)
}
