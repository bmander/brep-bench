//! A torus about z centred on the origin, its tube's middle circle radius 5, its tube 4.99: a hole
//! 0.02 across.
use cad_kernel::build::torus;
use cad_kernel::topo::Brep;

const MAJOR: f64 = 5.;
const MINOR: f64 = 4.99;

pub fn build() -> Result<Brep, String> {
    torus([0., 0., 0.], [0., 0., 1.], MAJOR, MINOR)
}
