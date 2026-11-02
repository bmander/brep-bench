//! A solid torus about z, its tube of radius 3 about a middle circle of radius 10, shrunk by 1.5.
use cad_kernel::build::torus;
use cad_kernel::offset::{shrunk, Join};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    shrunk(&torus([0., 0., 0.], [0., 0., 1.], 10., 3.)?, 1.5, Join::Round)
}
