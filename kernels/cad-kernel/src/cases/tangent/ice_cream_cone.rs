//! A ball of radius 15 at the origin united with a cone on z, apex (0, 0, −25), base the disc of
//! radius 12 at z = −9, whose side is tangent to the sphere along the rim of its base.
use crate::sketch::polygon;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{ball_section, revolve};
use cad_kernel::geom::Frame;
use cad_kernel::topo::Brep;
use std::f64::consts::TAU;

const BALL_RADIUS: f64 = 15.;
const APEX: f64 = -25.;
const BASE: f64 = -9.;
const BASE_RADIUS: f64 = 12.;

pub fn build() -> Result<Brep, String> {
    let (centre, up) = ([0., 0., 0.], [0., 0., 1.]);
    // workaround: no sphere primitive, no cone primitive
    // the ball: its half disc in the xz plane turned once about z
    let xz = Frame::new(centre, [0., -1., 0.], [1., 0., 0.]);
    let ball = revolve(&ball_section(&xz, BALL_RADIUS), centre, xz.y, TAU)?;
    // the cone: its meridian, the triangle from the apex out to the base's rim and back along the
    // base to the axis, turned once about z
    let meridian = polygon(&[[0., 0., APEX], [BASE_RADIUS, 0., BASE], [0., 0., BASE]]);
    let cone = revolve(&meridian, centre, up, TAU)?;
    Ok(combined(&ball, &cone, Op::Union, 0.)?)
}
