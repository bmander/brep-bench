//! A ball of radius 5 at the origin, bored right through along z by a hole of radius 3.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{ball_section, disc, prism, revolve};
use cad_kernel::geom::Frame;
use cad_kernel::topo::Brep;
use std::f64::consts::TAU;

const RADIUS: f64 = 5.;
const HOLE_RADIUS: f64 = 3.;

// workaround: no sphere primitive
pub fn build() -> Result<Brep, String> {
    let centre = [0., 0., 0.];
    let xz = Frame::new(centre, [0., -1., 0.], [1., 0., 0.]);
    let ball = revolve(&ball_section(&xz, RADIUS), centre, xz.y, TAU)?;
    // a drill reaching past the ball both ways along z
    let drill = prism(&disc([0., 0., -2. * RADIUS], [0., 0., 1.], HOLE_RADIUS), 0., 4. * RADIUS)?;
    Ok(combined(&ball, &drill, Op::Cut, 0.)?)
}
