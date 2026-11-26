//! The cube from (−10, −10, −10) to (10, 10, 10) less the oblate spheroid (x² + y²)/36 + z²/9 ≤ 1,
//! a closed cavity at its middle, shrunk by 1: the cube shrinks to (−9, −9, −9)–(9, 9, 9), and the
//! cavity grows to the spheroid's parallel body at distance 1.
use crate::cases::geometry::oblate_spheroid;
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::offset::{shrunk, Join};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let cube = block([-10., -10., -10.], [10., 10., 10.])?;
    let hollow = combined(&cube, &oblate_spheroid::build()?, Op::Cut, 0.)?;
    shrunk(&hollow, 1., Join::Round)
}
