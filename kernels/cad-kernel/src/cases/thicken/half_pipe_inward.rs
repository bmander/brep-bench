//! The half of the cylinder of radius 5 about z where y ≥ 0, from z = 0 to 10, a sheet, thickened
//! by 2 toward the axis.
use crate::sketch::{faces_through, half_pipe};
use cad_kernel::offset::{sheet_of, thickened, Join};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let solid = half_pipe()?;
    let faces = faces_through(&solid, &[[0., 5., 5.]])?;
    thickened(&sheet_of(&solid, &faces)?, [-2., 0.], Join::Round)
}
