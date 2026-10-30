//! A shaft on z: radius 5 from z = 0 to 10, and radius 3 from z = 10 to 20, one solid.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::topo::Brep;

const UP: [f64; 3] = [0., 0., 1.];

pub fn build() -> Result<Brep, String> {
    let lower = prism(&disc([0., 0., 0.], UP, 5.), 0., 10.)?;
    let upper = prism(&disc([0., 0., 10.], UP, 3.), 0., 10.)?;
    Ok(combined(&lower, &upper, Op::Union, 0.)?)
}
