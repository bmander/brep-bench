//! The upper half of the sphere of radius 5 about the origin, a sheet, thickened by 1 to each side.
use crate::sketch::faces_through;
use cad_kernel::build::{kept, sphere};
use cad_kernel::offset::{sheet_of, thickened, Join};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    // the upper half ball's sphere
    let solid = kept(&sphere([0., 0., 0.], [0., 0., 1.], 5.)?, [0., 0., 0.], [0., 0., 1.], 0.)?;
    let faces = faces_through(&solid, &[[3., 0., 4.]])?;
    thickened(&sheet_of(&solid, &faces)?, [-1., 1.], Join::Round)
}
