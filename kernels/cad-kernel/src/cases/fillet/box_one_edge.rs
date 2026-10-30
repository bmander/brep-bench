//! A cube of side 10 at the origin, its edge where x = 10 meets y = 10 rounded to radius 2.
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::fillet::roll;
use cad_kernel::geom::tolerance;
use cad_kernel::topo::Brep;

const SIDE: f64 = 10.;
const RADIUS: f64 = 2.;

pub fn build() -> Result<Brep, String> {
    let cube = block([0., 0., 0.], [SIDE, SIDE, SIDE])?;
    // the edge is picked by a point on it; the kernel finds the two faces either side, rolls the
    // ball between them, and builds the piece of material between the edge and the round
    let round = roll(&cube, [SIDE, SIDE, SIDE / 2.], RADIUS, tolerance(cube.size()))?;
    // a convex edge's round takes its piece away; a concave one's would fill it in
    let op = if round.concave { Op::Union } else { Op::Cut };
    Ok(combined(&cube, &round.piece, op, 0.)?)
}
