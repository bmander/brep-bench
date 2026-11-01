//! A ball of radius 15 at the origin united with a cone on z, apex (0, 0, −25), base the disc of
//! radius 12 at z = −9, whose side is tangent to the sphere along the rim of its base.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{cone, sphere};
use cad_kernel::topo::Brep;

const BALL_RADIUS: f64 = 15.;
const APEX: f64 = -25.;
const BASE: f64 = -9.;
const BASE_RADIUS: f64 = 12.;

pub fn build() -> Result<Brep, String> {
    let (centre, up) = ([0., 0., 0.], [0., 0., 1.]);
    // the ball, its poles on the vertical through its centre
    let ball = sphere(centre, up, BALL_RADIUS)?;
    // the cone: from its apex on the axis, below the ball, up to its base
    let cone = cone([0., 0., APEX], up, 0., BASE_RADIUS, BASE - APEX)?;
    Ok(combined(&ball, &cone, Op::Union, 0.)?)
}
