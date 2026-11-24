//! The box from the origin to (20, 10, 10) with a slot cut into its side y = 0 top to bottom, its
//! flats x = 8 and x = 12 running in to y = 5 and closed there by the half cylinder of radius 2
//! about the line through (10, 5); the slot's three faces then deleted, the side y = 0 running on
//! across the slot's mouth and the top and the bottom closing over its outline.
//!
//! The edit is the kernel's: `defeature::deleted` runs the faces round the deleted ones on along
//! their own surfaces and closes the gap they part off.
use crate::sketch::{block, cylinder, deleting, faces_through};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    // the slot's cutter: the box from x = 8 to 12 in from past the side to y = 5, and the
    // cylinder of radius 2 closing it, both running past the top and the bottom
    let cutter = combined(&block([8., -1., -1.], [12., 5., 11.])?,
        &cylinder([10., 5., -1.], 2., 12.)?, Op::Union, 0.)?;
    let b = combined(&block([0., 0., 0.], [20., 10., 10.])?, &cutter, Op::Cut, 0.)?;
    let slot = faces_through(&b, &[[8., 2., 5.], [12., 2., 5.], [10., 7., 5.]])?;
    deleting(&b, &slot)
}
