//! The cylinder of radius 6 on the z axis from z = 0 to z = 10, both its circular edges rounded to
//! radius 2, shrunk by 3: every point of it at least 3 inside it.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::fillet::roll;
use cad_kernel::geom::{tolerance, V};
use cad_kernel::offset::{shrunk, Join};
use cad_kernel::topo::Brep;

const RADIUS: f64 = 6.;
const HEIGHT: f64 = 10.;
const ROUND: f64 = 2.;
const SHRINK: f64 = 3.;

/// `part` with the edge through `at` rounded to `ROUND`: the kernel's piece taken off a convex
/// edge, filled in at a concave one.
fn rounded(part: &Brep, at: V) -> Result<Brep, String> {
    let round = roll(part, at, ROUND, tolerance(part.size()))?;
    let op = if round.concave { Op::Union } else { Op::Cut };
    Ok(combined(part, &round.piece, op, 0.)?)
}

pub fn build() -> Result<Brep, String> {
    let cylinder = prism(&disc([0., 0., 0.], [0., 0., 1.], RADIUS), 0., HEIGHT)?;
    // each rim is picked by a point on it, the top's and then the bottom's
    let puck = rounded(&rounded(&cylinder, [0., RADIUS, HEIGHT])?, [0., RADIUS, 0.])?;
    shrunk(&puck, SHRINK, Join::Round)
}
