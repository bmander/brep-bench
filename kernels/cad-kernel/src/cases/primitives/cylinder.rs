//! A solid cylinder of radius 5 and height 10 on the z axis, from z = 0 to z = 10.
use cad_kernel::build::{disc, prism};
use cad_kernel::topo::Brep;

const RADIUS: f64 = 5.;
const HEIGHT: f64 = 10.;

pub fn build() -> Result<Brep, String> {
    // the disc about the origin square to z, swept up its normal
    prism(&disc([0., 0., 0.], [0., 0., 1.], RADIUS), 0., HEIGHT)
}
