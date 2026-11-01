//! A floor (z = 0, 10 by 8) and a wall (x = 0, 8 by 10) meeting at a right angle along y, a sheet,
//! thickened by 1 to the inside of the crease (+z and +x).
use crate::sketch::{block, faces_through};
use cad_kernel::offset::{sheet_of, thickened, Join};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    // the sheet's faces lifted off the solid they bound, their normals its outward ones
    let solid = block([0., 0., 0.], [10., 8., 10.])?;
    let faces = faces_through(&solid, &[[5., 4., 0.], [0., 4., 5.]])?;
    thickened(&sheet_of(&solid, &faces)?, [-1., 0.], Join::Round)
}
