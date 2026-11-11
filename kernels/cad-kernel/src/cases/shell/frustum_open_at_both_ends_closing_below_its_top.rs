//! A cone frustum on the z axis, the disk of radius 6 at z = 0 up to the disk of radius 3 at z = 4, shelled inward to a wall 2.7 thick, open at both ends: the side moved inward meets the base's plane in the circle of radius 2.625 and closes to its apex at (0, 0, 3.5), below the top, so the cavity is a cone open at the base only and the top stays whole.
use crate::sketch::faces_through;
use cad_kernel::build::cone;
use cad_kernel::offset::{shelled, Join, Side};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let body = cone([0., 0., 0.], [0., 0., 1.], 6., 3., 4.)?;
    let open = faces_through(&body, &[[1., 1., 0.], [1., 1., 4.]])?;
    shelled(&body, &open, 2.7, Side::In, Join::Round)
}
