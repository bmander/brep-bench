//! A solid ball of radius 5 centred on the origin.
use cad_kernel::build::{ball_section, revolve};
use cad_kernel::geom::Frame;
use cad_kernel::topo::Brep;
use std::f64::consts::TAU;

const RADIUS: f64 = 5.;

pub fn build() -> Result<Brep, String> {
    // cad-kernel has no sphere primitive: a ball is its half disc turned once about the line it is
    // shut along. The frame's plane is xz (its z is −y) and its y, that line, is the z axis.
    let centre = [0., 0., 0.];
    let xz = Frame::new(centre, [0., -1., 0.], [1., 0., 0.]);
    revolve(&ball_section(&xz, RADIUS), centre, xz.y, TAU)
}
