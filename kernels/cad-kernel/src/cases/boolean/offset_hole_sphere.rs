//! A ball of radius 20 at the origin, bored along y by a hole of radius 6 on the line x = 5, z = 0.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{ball_section, disc, prism, revolve};
use cad_kernel::geom::Frame;
use cad_kernel::topo::Brep;
use std::f64::consts::TAU;

const RADIUS: f64 = 20.;
const HOLE_RADIUS: f64 = 6.;
const HOLE_X: f64 = 5.;

pub fn build() -> Result<Brep, String> {
    let centre = [0., 0., 0.];
    let xz = Frame::new(centre, [0., -1., 0.], [1., 0., 0.]);
    let ball = revolve(&ball_section(&xz, RADIUS), centre, xz.y, TAU)?;
    // a drill along y reaching past the ball both ways
    let start = [HOLE_X, -2. * RADIUS, 0.];
    let drill = prism(&disc(start, [0., 1., 0.], HOLE_RADIUS), 0., 4. * RADIUS)?;
    Ok(combined(&ball, &drill, Op::Cut, 0.)?)
}
