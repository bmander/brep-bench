//! The box from the origin to (10, 8, 6), grown by 1: every point within 1 of it.
use crate::sketch::block;
use cad_kernel::offset::{grown, Join};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    grown(&block([0., 0., 0.], [10., 8., 6.])?, 1., Join::Round)
}
