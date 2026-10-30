//! A cylinder of radius 5 on z from z = 0 to 10, less the ball of radius 5 about (0, 0, 10), the
//! middle of its top face: a hemispherical dish meeting the wall in a knife edge.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{ball_section, disc, prism, revolve};
use cad_kernel::geom::Frame;
use cad_kernel::topo::Brep;
use std::f64::consts::TAU;

const RADIUS: f64 = 5.;
const HEIGHT: f64 = 10.;

pub fn build() -> Result<Brep, String> {
    let cylinder = prism(&disc([0., 0., 0.], [0., 0., 1.], RADIUS), 0., HEIGHT)?;
    // the ball: its half disc in the xz plane turned once about z, its equator the top's rim
    let centre = [0., 0., HEIGHT];
    let xz = Frame::new(centre, [0., -1., 0.], [1., 0., 0.]);
    let ball = revolve(&ball_section(&xz, RADIUS), centre, xz.y, TAU)?;
    Ok(combined(&cylinder, &ball, Op::Cut, 0.)?)
}
