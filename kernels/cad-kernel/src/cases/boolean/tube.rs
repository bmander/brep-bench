//! A cylinder of radius 5 on z from z = 0 to 10, bored along its axis by a hole of radius 3.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::topo::Brep;

const UP: [f64; 3] = [0., 0., 1.];
const HEIGHT: f64 = 10.;

pub fn build() -> Result<Brep, String> {
    let rod = prism(&disc([0., 0., 0.], UP, 5.), 0., HEIGHT)?;
    // the drill runs past both ends, from z = −1 to z = 11
    let drill = prism(&disc([0., 0., -1.], UP, 3.), 0., HEIGHT + 2.)?;
    Ok(combined(&rod, &drill, Op::Cut, 0.)?)
}
