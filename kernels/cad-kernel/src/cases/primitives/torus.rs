//! A solid torus about z centred on the origin, its tube's middle circle radius 10, its tube 2.
use cad_kernel::build::torus;
use cad_kernel::topo::Brep;

const MAJOR: f64 = 10.;
const MINOR: f64 = 2.;

pub fn build() -> Result<Brep, String> {
    torus([0., 0., 0.], [0., 0., 1.], MAJOR, MINOR)
}
