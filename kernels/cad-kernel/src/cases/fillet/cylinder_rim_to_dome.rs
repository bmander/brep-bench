//! A cylinder of radius 5 on z from z = 0 to 10, the rim of its top face rounded to radius 5, its
//! own radius: the top shrinks to a point and the round is a hemisphere.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::fillet::roll;
use cad_kernel::geom::tolerance;
use cad_kernel::topo::Brep;

const RADIUS: f64 = 5.;
const HEIGHT: f64 = 10.;
const ROUND: f64 = RADIUS;

pub fn build() -> Result<Brep, String> {
    // the disc swept up, its circle's face named `side` (the caps are `far` and `near`)
    let mut section = disc([0., 0., 0.], [0., 0., 1.], RADIUS);
    section.names = vec![vec!["side".into()]];
    let cylinder = prism(&section, 0., HEIGHT)?;
    // the rim is picked by a point on it; the ball that rolls round it has its centre on the axis
    let round = roll(&cylinder, [0., RADIUS, HEIGHT], ROUND, tolerance(cylinder.size()))?;
    let op = if round.concave { Op::Union } else { Op::Cut };
    Ok(combined(&cylinder, &round.piece, op, 0.)?)
}
