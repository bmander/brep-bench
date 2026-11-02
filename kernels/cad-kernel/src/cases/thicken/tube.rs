//! The whole cylinder of radius 5 about z from z = 0 to 10, a sheet closing round on itself,
//! thickened by 1 away from the axis.
use crate::sketch::faces_through;
use cad_kernel::build::{disc, prism};
use cad_kernel::offset::{sheet_of, thickened, Join};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let solid = prism(&disc([0., 0., 0.], [0., 0., 1.], 5.), 0., 10.)?;
    let faces = faces_through(&solid, &[[0., 5., 5.]])?;
    thickened(&sheet_of(&solid, &faces)?, [0., 1.], Join::Round)
}
