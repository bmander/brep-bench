//! The rectangle from (0, 0, 0) to (10, 8, 0), a sheet, thickened by 2 to its +z side.
use crate::sketch::{block, faces_through};
use cad_kernel::offset::{sheet_of, thickened, Join};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    // the sheet: the top of a block, its normal +z
    let solid = block([0., 0., -1.], [10., 8., 0.])?;
    let faces = faces_through(&solid, &[[5., 4., 0.]])?;
    thickened(&sheet_of(&solid, &faces)?, [0., 2.], Join::Round)
}
