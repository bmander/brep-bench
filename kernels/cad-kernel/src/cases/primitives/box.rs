//! A rectangular block 10 long in x, 20 in y and 30 in z, from the origin to (10, 20, 30).
use crate::sketch::polygon;
use cad_kernel::build::prism;
use cad_kernel::topo::Brep;

const LENGTH: f64 = 10.;
const WIDTH: f64 = 20.;
const HEIGHT: f64 = 30.;

pub fn build() -> Result<Brep, String> {
    // the rectangle on the xy plane, walked counter-clockwise so it faces up, swept up its normal
    let floor = polygon(&[[0., 0., 0.], [LENGTH, 0., 0.], [LENGTH, WIDTH, 0.], [0., WIDTH, 0.]]);
    prism(&floor, 0., HEIGHT)
}
