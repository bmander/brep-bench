//! The side of the frustum about z from radius 6 at z = 0 to 3 at z = 4, a sheet, thickened by 1
//! away from the axis.
use crate::sketch::faces_through;
use cad_kernel::build::cone;
use cad_kernel::offset::{sheet_of, thickened, Join};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let solid = cone([0., 0., 0.], [0., 0., 1.], 6., 3., 4.)?;
    let faces = faces_through(&solid, &[[4.5, 0., 2.]])?;
    thickened(&sheet_of(&solid, &faces)?, [0., 1.], Join::Round)
}
