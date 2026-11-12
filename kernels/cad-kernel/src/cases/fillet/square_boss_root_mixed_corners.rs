//! The plate from the origin to (40, 30, 5) with the square boss from (12, 7, 5) to (28, 23, 15)
//! joined to its top, the boss's four upright edges rounded to radius 3, then its root — a square
//! of side 16 with its corners rounded to 3, one chain — rounded to radius 2, concave: a quarter
//! of a torus's tube about each upright round's axis. Its top left sharp.
//!
//! Refused, by the kernel, at the second step: the four upright rounds are rolled and taken off,
//! but the root's first stretch, along the side y = 7, is not. That side now ends at x = 15 and 25
//! in the seams where it goes on tangent into the upright rounds, and there 0.10.0 refuses the run:
//! "the fillet of `face 5` with `face 6` runs on smoothly into `face 12`, but its contact crosses
//! their seam in another section than its edge does (2.00e0 off it there): no one section joins the
//! run to the piece going on".
use crate::sketch::{block, named};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::fillet::roll;
use cad_kernel::geom::{tolerance, V};
use cad_kernel::topo::Brep;
use std::f64::consts::FRAC_1_SQRT_2;

const PLATE: V = [40., 30., 5.];
const BOSS: [V; 2] = [[12., 7., 5.], [28., 23., 15.]];
const UPRIGHT: f64 = 3.;
const ROOT: f64 = 2.;

/// The round of radius `r` of the edge of `b` nearest `at` taken off (convex) or added (concave).
/// Every face is named apart first: a roll reads faces of one name as pieces of one face, and the
/// faces the Booleans leave share theirs.
fn rounded(b: &Brep, at: V, r: f64) -> Result<Brep, String> {
    let b = &named(b.clone());
    let round = roll(b, at, r, tolerance(b.size()))
        .map_err(|e| format!("the edge at {at:?}: {e}"))?;
    let op = if round.concave { Op::Union } else { Op::Cut };
    combined(b, &round.piece, op, 0.).map_err(|e| format!("the round at {at:?} applied: {e}"))
}

pub fn build() -> Result<Brep, String> {
    let part = combined(&block([0.; 3], PLATE)?, &block(BOSS[0], BOSS[1])?, Op::Union, 0.)?;
    let ([x0, y0, z0], [x1, y1, z1]) = (BOSS[0], BOSS[1]);
    // 1. the four upright edges, each picked by its middle
    let zm = 0.5 * (z0 + z1);
    let uprights = [[x0, y0, zm], [x1, y0, zm], [x1, y1, zm], [x0, y1, zm]];
    let part = uprights.into_iter().try_fold(part, |b, at| rounded(&b, at, UPRIGHT))?;
    // 2. the root, one tangent chain of eight edges: fillet::roll rolls the ball along the run
    // between two faces, one edge here, so each in turn, picked by its middle — the sides' at
    // their middles, the quarter circles' halfway round
    let (xm, ym) = (0.5 * (x0 + x1), 0.5 * (y0 + y1));
    let off = UPRIGHT * (1. - FRAC_1_SQRT_2);
    let root: [V; 8] = [
        [xm, y0, z0], [x1 - off, y0 + off, z0], [x1, ym, z0], [x1 - off, y1 - off, z0],
        [xm, y1, z0], [x0 + off, y1 - off, z0], [x0, ym, z0], [x0 + off, y0 + off, z0],
    ];
    root.into_iter().try_fold(part, |b, at| rounded(&b, at, ROOT))
}
