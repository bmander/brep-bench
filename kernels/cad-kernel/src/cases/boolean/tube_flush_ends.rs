//! A cylinder of radius 5 on z from z = 0 to 10, less one of radius 3 over exactly the same span.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::topo::Brep;

const UP: [f64; 3] = [0., 0., 1.];
const HEIGHT: f64 = 10.;

pub fn build() -> Result<Brep, String> {
    let rod = prism(&disc([0., 0., 0.], UP, 5.), 0., HEIGHT)?;
    let core = prism(&disc([0., 0., 0.], UP, 3.), 0., HEIGHT)?;
    Ok(combined(&rod, &core, Op::Cut, 0.)?)
}
