//! Five faces of the cube from the origin to (10, 10, 10), all but its top, a sheet, thickened by 1
//! out of the cube, its edges rounded.
use crate::sketch::{block, faces_through};
use cad_kernel::offset::{sheet_of, thickened, Join};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    // the open box: the cube's faces but its top, their normals out of the cube
    let solid = block([0., 0., 0.], [10., 10., 10.])?;
    let faces = faces_through(&solid, &[[5., 5., 0.], [0., 5., 5.], [10., 5., 5.], [5., 0., 5.], [5., 10., 5.]])?;
    thickened(&sheet_of(&solid, &faces)?, [0., 1.], Join::Round)
}
