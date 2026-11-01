//! A torus about z, radii 10 and 3, united with a drum of radius 13 on z from z = −1 to 1, whose
//! wall the torus touches from inside along its outer equator.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism, torus};
use cad_kernel::topo::Brep;

const MAJOR: f64 = 10.;
const MINOR: f64 = 3.;
const DRUM_RADIUS: f64 = 13.;
const DRUM_HALF_HEIGHT: f64 = 1.;

pub fn build() -> Result<Brep, String> {
    let up = [0., 0., 1.];
    let torus = torus([0., 0., 0.], up, MAJOR, MINOR)?;
    let foot = [0., 0., -DRUM_HALF_HEIGHT];
    let drum = prism(&disc(foot, up, DRUM_RADIUS), 0., 2. * DRUM_HALF_HEIGHT)?;
    Ok(combined(&torus, &drum, Op::Union, 0.)?)
}
