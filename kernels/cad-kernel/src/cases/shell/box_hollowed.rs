//! The box from the origin to (10, 8, 6) hollowed to a wall 1 thick, closed.
use crate::sketch::block;
use cad_kernel::offset::{shelled, Join, Side};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let body = block([0., 0., 0.], [10., 8., 6.])?;
    shelled(&body, &[], 1., Side::In, Join::Round)
}
