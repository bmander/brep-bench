//! The cube from (−10, −10, −10) to (10, 10, 10) less the oblate spheroid (x² + y²)/36 + z²/9 ≤ 1,
//! a closed cavity at its middle, grown by 1: the cube grows as a box does, and the cavity shrinks
//! to the spheroid's inner parallel body at distance 1.
use crate::cases::geometry::oblate_spheroid;
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::offset::{grown, Join};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let cube = block([-10., -10., -10.], [10., 10., 10.])?;
    let hollow = combined(&cube, &oblate_spheroid::build()?, Op::Cut, 0.)?;
    grown(&hollow, 1., Join::Round)
}
