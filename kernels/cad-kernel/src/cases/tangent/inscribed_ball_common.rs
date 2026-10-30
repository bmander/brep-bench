//! The solid common to a cube of side 10 at the origin and its inscribed ball, radius 5 about
//! (5, 5, 5), which touches each of the cube's faces at the middle of the face.
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{ball_section, revolve};
use cad_kernel::geom::Frame;
use cad_kernel::topo::Brep;
use std::f64::consts::TAU;

const SIDE: f64 = 10.;
const RADIUS: f64 = 5.;

pub fn build() -> Result<Brep, String> {
    let cube = block([0., 0., 0.], [SIDE, SIDE, SIDE])?;
    // workaround: no sphere primitive
    // the ball: its half disc in the vertical plane through its centre, parallel to xz, turned
    // once about the vertical through the centre
    let centre = [SIDE / 2.; 3];
    let xz = Frame::new(centre, [0., -1., 0.], [1., 0., 0.]);
    let ball = revolve(&ball_section(&xz, RADIUS), centre, xz.y, TAU)?;
    Ok(combined(&cube, &ball, Op::Common, 0.)?)
}
