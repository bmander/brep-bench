//! The sphere of radius 5 about the origin, a closed sheet, thickened by 1 outward.
use crate::sketch::faces_through;
use cad_kernel::build::sphere;
use cad_kernel::offset::{sheet_of, thickened, Join};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let solid = sphere([0., 0., 0.], [0., 0., 1.], 5.)?;
    let faces = faces_through(&solid, &[[3., 0., 4.]])?;
    thickened(&sheet_of(&solid, &faces)?, [0., 1.], Join::Round)
}
