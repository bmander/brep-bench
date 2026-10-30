//! A ball of radius 5 centred on the origin, with everything below the xy plane cut away.
use cad_kernel::build::{ball_section, kept, revolve};
use cad_kernel::geom::Frame;
use cad_kernel::topo::Brep;
use std::f64::consts::TAU;

const RADIUS: f64 = 5.;

pub fn build() -> Result<Brep, String> {
    let centre = [0., 0., 0.];
    // workaround: no sphere primitive
    // the ball: its half disc in the xz plane turned once about z
    let xz = Frame::new(centre, [0., -1., 0.], [1., 0., 0.]);
    let ball = revolve(&ball_section(&xz, RADIUS), centre, xz.y, TAU)?;
    // the part above the xy plane
    kept(&ball, centre, [0., 0., 1.], 0.)
}
