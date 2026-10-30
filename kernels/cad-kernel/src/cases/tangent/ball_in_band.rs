//! A ball of radius 5 centred on the origin, united with a cylinder of the same radius on the z
//! axis from z = −3 to z = 3, which the ball touches from inside along its equator.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{ball_section, disc, prism, revolve};
use cad_kernel::geom::Frame;
use cad_kernel::topo::Brep;
use std::f64::consts::TAU;

const RADIUS: f64 = 5.;
const HALF_HEIGHT: f64 = 3.;

pub fn build() -> Result<Brep, String> {
    // workaround: no sphere primitive
    // the ball: its half disc in the xz plane turned once about z
    let centre = [0., 0., 0.];
    let xz = Frame::new(centre, [0., -1., 0.], [1., 0., 0.]);
    let ball = revolve(&ball_section(&xz, RADIUS), centre, xz.y, TAU)?;
    // the band: the disc at z = −3 swept up to z = 3, its wall tangent to the sphere at z = 0
    let band = prism(&disc([0., 0., -HALF_HEIGHT], [0., 0., 1.], RADIUS), 0., 2. * HALF_HEIGHT)?;
    Ok(combined(&ball, &band, Op::Union, 0.)?)
}
