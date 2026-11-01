//! The box from the origin to (10, 8, 6), shrunk by 1: every point at least 1 inside it.
use crate::sketch::block;
use cad_kernel::offset::{shrunk, Join};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    shrunk(&block([0., 0., 0.], [10., 8., 6.])?, 1., Join::Round)
}
