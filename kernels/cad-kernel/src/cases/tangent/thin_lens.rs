//! Two balls of radius 5, about the origin and about (9.999999, 0, 0): the lens common to both,
//! a millionth thick.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{ball_section, revolve};
use cad_kernel::geom::{Frame, V};
use cad_kernel::topo::Brep;
use std::f64::consts::TAU;

const RADIUS: f64 = 5.;
const APART: f64 = 9.999999;

// workaround: no sphere primitive
/// The ball of radius `RADIUS` about `centre`: its half disc in the xz plane turned once about the
/// vertical through the centre.
fn ball(centre: V) -> Result<Brep, String> {
    let xz = Frame::new(centre, [0., -1., 0.], [1., 0., 0.]);
    revolve(&ball_section(&xz, RADIUS), centre, xz.y, TAU)
}

pub fn build() -> Result<Brep, String> {
    let (first, second) = (ball([0., 0., 0.])?, ball([APART, 0., 0.])?);
    Ok(combined(&first, &second, Op::Common, 0.)?)
}
