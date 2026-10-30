//! The rectangle x from 6 to 10, z from 0 to 4 in the xz plane, revolved a full turn about z.
use crate::sketch::polygon;
use cad_kernel::build::revolve;
use cad_kernel::topo::Brep;
use std::f64::consts::TAU;

const INNER: f64 = 6.;
const OUTER: f64 = 10.;
const HEIGHT: f64 = 4.;

pub fn build() -> Result<Brep, String> {
    let section = polygon(&[[INNER, 0., 0.], [OUTER, 0., 0.], [OUTER, 0., HEIGHT], [INNER, 0., HEIGHT]]);
    revolve(&section, [0., 0., 0.], [0., 0., 1.], TAU)
}
