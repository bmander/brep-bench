//! The plate from (0, 0, 0) to (10, 10, 2) with the square hole from (3, 3) to (7, 7) through it,
//! grown by 1: the hole's four upright edges are concave and stay sharp.
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::offset::{grown, Join};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let plate = block([0., 0., 0.], [10., 10., 2.])?;
    // the punch runs 1 past both faces of the plate
    let punch = block([3., 3., -1.], [7., 7., 3.])?;
    grown(&combined(&plate, &punch, Op::Cut, 0.)?, 1., Join::Round)
}
