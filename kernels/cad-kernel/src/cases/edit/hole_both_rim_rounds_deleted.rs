//! The box from the origin to (20, 10, 10), drilled through along z by a hole of radius 2 about the
//! line x = 10, y = 5, both of the hole's rims rounded to radius 1 by quarters of the tori whose
//! tubes have radius 1 about the circles of radius 3 at z = 9 and at z = 1; both rounds then
//! deleted together, the top and the bottom running on inward from radius 3 and the wall up from
//! z = 9 and down from z = 1 until they meet in the circles of radius 2 at z = 10 and at z = 0.
//!
//! The rounded hole is the box, a `prism`, drilled by a `prism` of a `disc`, each rim rolled by
//! `fillet::roll` and its piece cut away (`sketch::rolled`); the two rounds are picked by a point
//! on each and deleted together (`defeature::deleted`).
use crate::sketch::{block, deleted_through, rolled};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::topo::Brep;
use std::f64::consts::FRAC_1_SQRT_2;

pub fn build() -> Result<Brep, String> {
    let drill = prism(&disc([10., 5., -1.], [0., 0., 1.], 2.), 0., 12.)?;
    let drilled = combined(&block([0., 0., 0.], [20., 10., 10.])?, &drill, Op::Cut, 0.)?;
    let rounded = rolled(&drilled, &[[12., 5., 10.], [12., 5., 0.]], 1.)?;
    // each round's middle, at 45° about its tube's circle of radius 3, toward its rim
    let (x, d) = (13. - FRAC_1_SQRT_2, FRAC_1_SQRT_2);
    deleted_through(&rounded, &[[x, 5., 9. + d], [x, 5., 1. - d]])
}
