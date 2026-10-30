//! A cylinder of radius 5 on z from z = 0 to 10, the rim of its top face rounded to radius 1.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::fillet::roll;
use cad_kernel::geom::tolerance;
use cad_kernel::topo::Brep;

const RADIUS: f64 = 5.;
const HEIGHT: f64 = 10.;
const ROUND: f64 = 1.;

pub fn build() -> Result<Brep, String> {
    let cylinder = prism(&disc([0., 0., 0.], [0., 0., 1.], RADIUS), 0., HEIGHT)?;
    // the rim is picked by a point on it; the kernel rolls the ball round the loop where the top
    // meets the side and builds the ring of material outside the round
    let round = roll(&cylinder, [0., RADIUS, HEIGHT], ROUND, tolerance(cylinder.size()))?;
    let op = if round.concave { Op::Union } else { Op::Cut };
    Ok(combined(&cylinder, &round.piece, op, 0.)?)
}
