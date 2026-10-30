//! A block (−10, −10, −10)–(10, 10, 0) with a ball of radius 5 at the origin cut from its top.
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{ball_section, revolve};
use cad_kernel::geom::Frame;
use cad_kernel::topo::Brep;
use std::f64::consts::TAU;

const RADIUS: f64 = 5.;

pub fn build() -> Result<Brep, String> {
    let stock = block([-10., -10., -10.], [10., 10., 0.])?;
    let centre = [0., 0., 0.];
    let xz = Frame::new(centre, [0., -1., 0.], [1., 0., 0.]);
    let ball = revolve(&ball_section(&xz, RADIUS), centre, xz.y, TAU)?;
    Ok(combined(&stock, &ball, Op::Cut, 0.)?)
}
