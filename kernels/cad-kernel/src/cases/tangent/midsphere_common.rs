//! The solid common to a cube of side 10 centred on the origin and the ball about the origin of
//! radius 5√2, which touches each of the cube's twelve edges at its midpoint.
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::sphere;
use cad_kernel::topo::Brep;
use std::f64::consts::SQRT_2;

const HALF_SIDE: f64 = 5.;

pub fn build() -> Result<Brep, String> {
    let cube = block([-HALF_SIDE; 3], [HALF_SIDE; 3])?;
    // the ball reaches the middle of an edge, 5√2 from the centre: its half disc in the xz plane
    // turned once about z
    let centre = [0., 0., 0.];
    // the ball reaches the middle of an edge, 5√2 from the centre; its poles on the vertical
    // through it
    let ball = sphere(centre, [0., 0., 1.], HALF_SIDE * SQRT_2)?;
    Ok(combined(&cube, &ball, Op::Common, 0.)?)
}
