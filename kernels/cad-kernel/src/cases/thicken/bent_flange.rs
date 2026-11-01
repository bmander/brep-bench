//! The profile from (−10, 0, 0) to the origin, round the quarter circle of radius 2 about (0, 0, 2)
//! to (2, 0, 2) and up to (2, 0, 12), swept 8 along y, a sheet, thickened by 1 away from the bend's
//! axis.
use crate::sketch::{faces_through, bent_flange};
use cad_kernel::offset::{sheet_of, thickened, Join};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let (solid, picks) = bent_flange()?;
    let faces = faces_through(&solid, &picks)?;
    thickened(&sheet_of(&solid, &faces)?, [0., 1.], Join::Round)
}
