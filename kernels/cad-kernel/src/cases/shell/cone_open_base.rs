//! A cone of base radius 6 at z = 0 and apex (0, 0, 8) shelled inward to a wall 1 thick, open at
//! its base.
use crate::sketch::faces_through;
use cad_kernel::build::cone;
use cad_kernel::offset::{shelled, Join, Side};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let body = cone([0., 0., 0.], [0., 0., 1.], 6., 0., 8.)?;
    let open = faces_through(&body, &[[1., 1., 0.]])?;
    shelled(&body, &open, 1., Side::In, Join::Round)
}
