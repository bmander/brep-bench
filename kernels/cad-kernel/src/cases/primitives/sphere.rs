//! A solid ball of radius 5 centred on the origin.
use cad_kernel::build::sphere;
use cad_kernel::topo::Brep;

const RADIUS: f64 = 5.;

pub fn build() -> Result<Brep, String> {
    sphere([0., 0., 0.], [0., 0., 1.], RADIUS)
}
