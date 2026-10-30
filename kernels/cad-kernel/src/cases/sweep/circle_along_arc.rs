//! A circle of radius 2 swept square to the arc of radius 10 about z from +x to +y; flat ends.
use cad_kernel::build::{disc, revolve};
use cad_kernel::topo::Brep;
use std::f64::consts::FRAC_PI_2;

const ARC_RADIUS: f64 = 10.;
const RADIUS: f64 = 2.;

pub fn build() -> Result<Brep, String> {
    // a sweep along an arc is a revolution: the section square to the arc where it starts (the
    // disc in the xz plane about (10, 0, 0)) turned a quarter about z
    let section = disc([ARC_RADIUS, 0., 0.], [0., 1., 0.], RADIUS);
    revolve(&section, [0., 0., 0.], [0., 0., 1.], FRAC_PI_2)
}
