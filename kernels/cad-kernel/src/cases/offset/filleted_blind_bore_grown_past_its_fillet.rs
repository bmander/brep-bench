//! The cylinder of radius 10 on the z axis from z = 0 to z = 8 with a blind bore of radius 5 in its
//! top down to a flat floor at z = 2, the concave edge where the bore's side meets its floor
//! filleted to radius 2, grown by 3: every point within 3 of it.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::fillet::roll;
use cad_kernel::geom::tolerance;
use cad_kernel::offset::{grown, Join};
use cad_kernel::topo::Brep;

const BLOCK_RADIUS: f64 = 10.;
const BLOCK_HEIGHT: f64 = 8.;
const BORE_RADIUS: f64 = 5.;
const FLOOR: f64 = 2.;
const FILLET: f64 = 2.;
const GROWTH: f64 = 3.;

pub fn build() -> Result<Brep, String> {
    let up = [0., 0., 1.];
    let block = prism(&disc([0., 0., 0.], up, BLOCK_RADIUS), 0., BLOCK_HEIGHT)?;
    // the drill runs from the floor up past the block's top
    let drill = prism(&disc([0., 0., FLOOR], up, BORE_RADIUS), 0., BLOCK_HEIGHT)?;
    let bored = combined(&block, &drill, Op::Cut, 0.)?;
    // the edge is picked by a point on it, where the bore's side meets its floor; the kernel
    // rolls the ball between the floor and the side round the loop
    let round = roll(&bored, [0., BORE_RADIUS, FLOOR], FILLET, tolerance(bored.size()))?;
    // a concave edge's round fills its piece in
    let op = if round.concave { Op::Union } else { Op::Cut };
    let filleted = combined(&bored, &round.piece, op, 0.)?;
    grown(&filleted, GROWTH, Join::Round)
}
