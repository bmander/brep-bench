//! A plate (−10, −10, 0)–(10, 10, 2) with a hole of radius 3 through it on z, both of the hole's
//! rims rounded to radius 1, half the plate's thickness: the two rounds meet at the hole's waist.
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::fillet::roll;
use cad_kernel::geom::{tolerance, V};
use cad_kernel::topo::Brep;

const HALF_SIDE: f64 = 10.;
const THICKNESS: f64 = 2.;
const HOLE_RADIUS: f64 = 3.;
const ROUND: f64 = 1.;

/// `part` with the edge through `at` rounded to `ROUND`: the kernel's piece taken off a convex
/// edge, filled in at a concave one.
fn rounded(part: &Brep, at: V) -> Result<Brep, String> {
    let round = roll(part, at, ROUND, tolerance(part.size()))?;
    let op = if round.concave { Op::Union } else { Op::Cut };
    Ok(combined(part, &round.piece, op, 0.)?)
}

pub fn build() -> Result<Brep, String> {
    // the plate's faces are `far`, the bottom, and `near`, the top
    let plate = block([-HALF_SIDE, -HALF_SIDE, 0.], [HALF_SIDE, HALF_SIDE, THICKNESS])?;
    // the drill, its side named `bore`, runs past both faces, from z = −1 to z = 3
    let mut drill = disc([0., 0., -1.], [0., 0., 1.], HOLE_RADIUS);
    drill.names = vec![vec!["bore".into()]];
    let part = combined(&plate, &prism(&drill, 0., THICKNESS + 2.)?, Op::Cut, 0.)?;
    // the rim on the top face, then the rim on the bottom, each picked by a point on it
    let part = rounded(&part, [0., HOLE_RADIUS, THICKNESS])?;
    rounded(&part, [0., HOLE_RADIUS, 0.])
}
