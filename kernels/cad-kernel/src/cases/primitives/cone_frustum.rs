//! A truncated cone on the z axis from z = 0 to z = 6, of radius 5 at its base and 2 at its top.
use crate::sketch::polygon;
use cad_kernel::build::revolve;
use cad_kernel::topo::Brep;
use std::f64::consts::TAU;

const BASE_RADIUS: f64 = 5.;
const TOP_RADIUS: f64 = 2.;
const HEIGHT: f64 = 6.;

pub fn build() -> Result<Brep, String> {
    // workaround: no cone primitive
    // its meridian in the xz plane, the axis one of its sides, turned once about z
    let meridian = polygon(&[
        [0., 0., 0.],
        [BASE_RADIUS, 0., 0.],
        [TOP_RADIUS, 0., HEIGHT],
        [0., 0., HEIGHT],
    ]);
    revolve(&meridian, [0., 0., 0.], [0., 0., 1.], TAU)
}
