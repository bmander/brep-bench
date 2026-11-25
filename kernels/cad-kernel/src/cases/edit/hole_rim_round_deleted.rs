//! The box from the origin to (20, 10, 10), drilled through along z by a hole of radius 2 about the
//! line x = 10, y = 5, the edge where the hole's wall meets the top rounded to radius 1 by a
//! quarter of the torus whose tube has radius 1 about the circle of radius 3 at z = 9; the round
//! then deleted, the top running on inward from radius 3 and the wall up from z = 9 until they meet
//! in the circle of radius 2 at z = 10.
//!
//! The rounded hole is the box, a `prism`, drilled by a `prism` of a `disc`, the top rim rolled by
//! `fillet::roll` and the round's piece cut away (`sketch::rolled`); the round is picked by a point
//! on it and deleted (`defeature::deleted`).
use crate::sketch::{block, deleted_through, rolled};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::topo::Brep;
use std::f64::consts::FRAC_1_SQRT_2;

pub fn build() -> Result<Brep, String> {
    let drill = prism(&disc([10., 5., -1.], [0., 0., 1.], 2.), 0., 12.)?;
    let drilled = combined(&block([0., 0., 0.], [20., 10., 10.])?, &drill, Op::Cut, 0.)?;
    let rounded = rolled(&drilled, &[[12., 5., 10.]], 1.)?;
    // the round's middle, at 45° about its tube's circle of radius 3 at z = 9, toward the rim
    deleted_through(&rounded, &[[13. - FRAC_1_SQRT_2, 5., 9. + FRAC_1_SQRT_2]])
}
