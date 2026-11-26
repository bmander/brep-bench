//! The block from (0, 0, 0) to (20, 20, 6) with the square pocket from (6, 6) to (14, 14) cut down
//! to z = 3, grown by 1: the pocket's rims round over and its concave edges stay sharp.
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::offset::{grown, Join};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let solid = block([0., 0., 0.], [20., 20., 6.])?;
    // the cutter from the pocket's floor on 1 past the top
    let pocket = block([6., 6., 3.], [14., 14., 7.])?;
    grown(&combined(&solid, &pocket, Op::Cut, 0.)?, 1., Join::Round)
}
