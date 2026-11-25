//! The box from the origin to (20, 10, 10) with a slot cut into its side y = 0 top to bottom, its
//! flats x = 8 and x = 12 running in to y = 5 and closed there by the half cylinder of radius 2
//! about the line through (10, 5); the slot's three faces then deleted, the side y = 0 running on
//! across the slot's mouth and the top and the bottom closing over its outline.
//!
//! The slotted block is the box, a `prism`, cut by the slot: a box from x = 8 to 12 running in from
//! past the side to y = 5, joined to the upright cylinder of radius 2 about the line through (10,
//! 5), both standing past the top and the bottom. The slot's two flats and its half cylinder are
//! picked by a point on each and deleted together (`defeature::deleted`).
use crate::sketch::{block, deleted_through};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let end = prism(&disc([10., 5., -1.], [0., 0., 1.], 2.), 0., 12.)?;
    let slot = combined(&block([8., -1., -1.], [12., 5., 11.])?, &end, Op::Union, 0.)?;
    let slotted = combined(&block([0., 0., 0.], [20., 10., 10.])?, &slot, Op::Cut, 0.)?;
    deleted_through(&slotted, &[[8., 2., 5.], [12., 2., 5.], [10., 7., 5.]])
}
