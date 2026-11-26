//! The plate from (−10, −10, 0) to (10, 10, 4) with a round recess of radius 3 sunk 1 into its top
//! about the z axis, grown by 4: the recess fills, and the grown top dips over it to a flat floor
//! at z = 7 ringed by the inside of a tube about the recess's rim.
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::offset::{grown, Join};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let plate = block([-10., -10., 0.], [10., 10., 4.])?;
    // the cutter from the recess's floor on 1 past the top
    let recess = prism(&disc([0., 0., 3.], [0., 0., 1.], 3.), 0., 2.)?;
    grown(&combined(&plate, &recess, Op::Cut, 0.)?, 4., Join::Round)
}
