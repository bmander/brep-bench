//! The plate of radius 8 on the z axis from z = −4 to z = 0 with a post standing on it, the
//! cylinder of radius 3 from z = 0 to z = 8, the concave edge where the post meets the plate
//! filleted to radius 2, grown by 3: every point within 3 of it.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::fillet::roll;
use cad_kernel::geom::tolerance;
use cad_kernel::offset::{grown, Join};
use cad_kernel::topo::Brep;

const PLATE_RADIUS: f64 = 8.;
const PLATE_DEPTH: f64 = 4.;
const POST_RADIUS: f64 = 3.;
const POST_HEIGHT: f64 = 8.;
const FILLET: f64 = 2.;
const GROWTH: f64 = 3.;

pub fn build() -> Result<Brep, String> {
    let up = [0., 0., 1.];
    let plate = prism(&disc([0., 0., -PLATE_DEPTH], up, PLATE_RADIUS), 0., PLATE_DEPTH)?;
    let post = prism(&disc([0., 0., 0.], up, POST_RADIUS), 0., POST_HEIGHT)?;
    let part = combined(&plate, &post, Op::Union, 0.)?;
    // the edge is picked by a point on it, the post's foot on the plate's top; the kernel rolls
    // the ball between the top and the post's side round the loop
    let round = roll(&part, [0., POST_RADIUS, 0.], FILLET, tolerance(part.size()))?;
    // a concave edge's round fills its piece in
    let op = if round.concave { Op::Union } else { Op::Cut };
    let filleted = combined(&part, &round.piece, op, 0.)?;
    grown(&filleted, GROWTH, Join::Round)
}
