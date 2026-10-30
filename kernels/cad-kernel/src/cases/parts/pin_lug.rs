//! A plate (0, −5, 0)–(80, 5, 40) with a pin hole of radius 8 through it along x = 60, z = 20.
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::topo::Brep;

const LENGTH: f64 = 80.;
const HEIGHT: f64 = 40.;
const HALF_THICKNESS: f64 = 5.;
const HOLE_RADIUS: f64 = 8.;
const HOLE_X: f64 = 60.;
const HOLE_Z: f64 = 20.;

pub fn build() -> Result<Brep, String> {
    let plate = block([0., -HALF_THICKNESS, 0.], [LENGTH, HALF_THICKNESS, HEIGHT])?;
    // the drill along y runs past both faces, from y = −6 to y = 6
    let start = [HOLE_X, -HALF_THICKNESS - 1., HOLE_Z];
    let drill = prism(&disc(start, [0., 1., 0.], HOLE_RADIUS), 0., 2. * HALF_THICKNESS + 2.)?;
    Ok(combined(&plate, &drill, Op::Cut, 0.)?)
}
