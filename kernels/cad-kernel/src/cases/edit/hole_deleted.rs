//! The box from the origin to (20, 10, 10), drilled right through along z by a hole of radius 3
//! about the line through (10, 5); the hole's wall then deleted, the top and the bottom closing
//! over the circles its two ends are.
//!
//! The drilled block is the box, a `prism`, cut by the drill, a `prism` of a `disc`; the wall is
//! picked by a point on it and deleted (`defeature::deleted`).
use crate::sketch::{block, deleted_through};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    // the drill runs past the floor and the top
    let drill = prism(&disc([10., 5., -1.], [0., 0., 1.], 3.), 0., 12.)?;
    let drilled = combined(&block([0., 0., 0.], [20., 10., 10.])?, &drill, Op::Cut, 0.)?;
    deleted_through(&drilled, &[[13., 5., 5.]])
}
