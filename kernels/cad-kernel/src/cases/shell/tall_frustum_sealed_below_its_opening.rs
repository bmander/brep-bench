//! A cone frustum on the z axis, the disk of radius 7 at z = 0 up to the disk of radius 1 at z = 8, shelled inward to a wall 2 thick, open at its top: the side moved inward closes to its apex at (0, 0, 6), 2 below the top, so the cavity is a closed cone and the top stays whole.
use crate::sketch::faces_through;
use cad_kernel::build::cone;
use cad_kernel::offset::{shelled, Join, Side};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let body = cone([0., 0., 0.], [0., 0., 1.], 7., 1., 8.)?;
    let open = faces_through(&body, &[[0.5, 0.5, 8.]])?;
    shelled(&body, &open, 2.0, Side::In, Join::Round)
}
