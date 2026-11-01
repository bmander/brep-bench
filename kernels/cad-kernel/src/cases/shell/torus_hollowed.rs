//! A torus of radius 2 about a middle circle of radius 6 hollowed to a wall 0.5 thick, closed.
use cad_kernel::build::torus;
use cad_kernel::offset::{shelled, Join, Side};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let body = torus([0., 0., 0.], [0., 0., 1.], 6., 2.)?;
    shelled(&body, &[], 0.5, Side::In, Join::Round)
}
