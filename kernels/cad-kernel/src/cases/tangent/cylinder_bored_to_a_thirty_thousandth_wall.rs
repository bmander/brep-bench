//! A cylinder of radius 5 and height 10 about the z axis from z = 0, less a cylinder of radius
//! 4.99997 on the same axis between the same two planes: a tube 10 tall whose wall is 0.00003
//! thick, its ends two thin annuli in the planes both cylinders' ends lie in.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::topo::Brep;

const UP: [f64; 3] = [0., 0., 1.];
const RADIUS: f64 = 5.;
const BORE: f64 = 4.99997;
const HEIGHT: f64 = 10.;

pub fn build() -> Result<Brep, String> {
    let rod = prism(&disc([0., 0., 0.], UP, RADIUS), 0., HEIGHT)?;
    // the bore over exactly the rod's span, its ends in the rod's end planes
    let bore = prism(&disc([0., 0., 0.], UP, BORE), 0., HEIGHT)?;
    Ok(combined(&rod, &bore, Op::Cut, 0.)?)
}
