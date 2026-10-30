//! An L-block, (0, 0), (10, 0), (10, 3), (3, 3), (3, 10), (0, 10) in xz extruded along y from 0 to
//! 10, its inside edge x = 3, z = 3 rounded to radius 7, which consumes both faces beside it.
use crate::sketch::polygon;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::prism;
use cad_kernel::fillet::roll;
use cad_kernel::geom::tolerance;
use cad_kernel::topo::Brep;

const LENGTH: f64 = 10.;
const INSIDE: f64 = 3.;
const ROUND: f64 = 7.;

pub fn build() -> Result<Brep, String> {
    let mut section = polygon(&[
        [0., 0., 0.],
        [10., 0., 0.],
        [10., 0., INSIDE],
        [INSIDE, 0., INSIDE],
        [INSIDE, 0., 10.],
        [0., 0., 10.],
    ]);
    // each side named by its edge of the section, the two inside ones `floor` (z = 3) and `wall`
    // (x = 3)
    let names = ["bottom", "end", "floor", "wall", "top", "back"];
    section.names = vec![names.map(String::from).to_vec()];
    // counter-clockwise in (x, z), so its normal is −y: ordinates −10 to 0 along it are y 10 to 0
    let block = prism(&section, -LENGTH, 0.)?;
    // the inside edge is picked by a point on it; the ball rolls in the corner touching both faces
    // at their far edges
    let round = roll(&block, [INSIDE, LENGTH / 2., INSIDE], ROUND, tolerance(block.size()))?;
    // a concave edge's round fills its piece in
    let op = if round.concave { Op::Union } else { Op::Cut };
    Ok(combined(&block, &round.piece, op, 0.)?)
}
