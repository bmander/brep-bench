//! A ball of radius 5 about the origin hollowed to a wall 1 thick, closed.
use cad_kernel::build::sphere;
use cad_kernel::offset::{shelled, Join, Side};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let body = sphere([0., 0., 0.], [0., 0., 1.], 5.)?;
    shelled(&body, &[], 1., Side::In, Join::Round)
}
