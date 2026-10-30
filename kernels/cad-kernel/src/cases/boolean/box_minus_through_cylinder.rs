//! A cube of side 10 at the origin, drilled through along z by a hole of radius 3 at (5, 5).
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::topo::Brep;

const SIDE: f64 = 10.;
const HOLE_RADIUS: f64 = 3.;

pub fn build() -> Result<Brep, String> {
    let cube = block([0., 0., 0.], [SIDE, SIDE, SIDE])?;
    // the drill runs past both faces, from z = −1 to z = 11
    let drill = prism(&disc([5., 5., -1.], [0., 0., 1.], HOLE_RADIUS), 0., SIDE + 2.)?;
    Ok(combined(&cube, &drill, Op::Cut, 0.)?)
}
