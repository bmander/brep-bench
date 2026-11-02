//! A cone on the z axis, base radius 3 at z = 0, apex at (0, 0, 4), grown by 1.
use cad_kernel::build::cone;
use cad_kernel::offset::{grown, Join};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    grown(&cone([0., 0., 0.], [0., 0., 1.], 3., 0., 4.)?, 1., Join::Round)
}
