//! The solid common to a cube of side 10 centred on the origin and the ball about the origin of
//! radius 5√2, which touches each of the cube's twelve edges at its midpoint.
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{ball_section, revolve};
use cad_kernel::geom::Frame;
use cad_kernel::topo::Brep;
use std::f64::consts::{SQRT_2, TAU};

const HALF_SIDE: f64 = 5.;

pub fn build() -> Result<Brep, String> {
    let cube = block([-HALF_SIDE; 3], [HALF_SIDE; 3])?;
    // workaround: no sphere primitive
    // the ball reaches the middle of an edge, 5√2 from the centre: its half disc in the xz plane
    // turned once about z
    let centre = [0., 0., 0.];
    let xz = Frame::new(centre, [0., -1., 0.], [1., 0., 0.]);
    let ball = revolve(&ball_section(&xz, HALF_SIDE * SQRT_2), centre, xz.y, TAU)?;
    Ok(combined(&cube, &ball, Op::Common, 0.)?)
}
