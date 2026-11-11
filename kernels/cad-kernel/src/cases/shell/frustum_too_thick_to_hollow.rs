//! A cone frustum on the z axis, the disk of radius 6 at z = 0 up to the disk of radius 3 at z = 4, shelled inward to a wall 3.5 thick, open at its top: the side moved inward closes at (0, 0, 13/6), below the base moved up to z = 3.5, so nothing is hollowed and the result is the frustum itself.
use crate::sketch::faces_through;
use cad_kernel::build::cone;
use cad_kernel::offset::{shelled, Join, Side};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let body = cone([0., 0., 0.], [0., 0., 1.], 6., 3., 4.)?;
    let open = faces_through(&body, &[[1., 1., 4.]])?;
    shelled(&body, &open, 3.5, Side::In, Join::Round)
}
