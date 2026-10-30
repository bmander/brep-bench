//! A solid torus about z centred on the origin, its tube's middle circle radius 10, its tube 2.
use cad_kernel::build::{disc, revolve};
use cad_kernel::topo::Brep;
use std::f64::consts::TAU;

const MAJOR: f64 = 10.;
const MINOR: f64 = 2.;

pub fn build() -> Result<Brep, String> {
    // workaround: no torus primitive
    // the tube's section: a disc in the xz plane (square to y) about (10, 0, 0), turned once about z
    let section = disc([MAJOR, 0., 0.], [0., 1., 0.], MINOR);
    revolve(&section, [0., 0., 0.], [0., 0., 1.], TAU)
}
