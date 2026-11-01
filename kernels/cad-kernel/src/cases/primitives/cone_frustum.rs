//! A truncated cone on the z axis from z = 0 to z = 6, of radius 5 at its base and 2 at its top.
use cad_kernel::build::cone;
use cad_kernel::topo::Brep;

const BASE_RADIUS: f64 = 5.;
const TOP_RADIUS: f64 = 2.;
const HEIGHT: f64 = 6.;

pub fn build() -> Result<Brep, String> {
    cone([0., 0., 0.], [0., 0., 1.], BASE_RADIUS, TOP_RADIUS, HEIGHT)
}
