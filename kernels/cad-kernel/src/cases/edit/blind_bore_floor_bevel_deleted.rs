//! The box from (−10, −10, 0) to (10, 10, 10) with a blind bore of radius 4 about the z axis sunk
//! from its top to a floor at z = 4, the edge where the floor meets the wall beveled: the wall
//! stops at z = 5 and a cone runs from there down and in to the floor, the disk of radius 3; the
//! bevel then deleted, the floor running on outward from radius 3 and the wall down from z = 5
//! until they meet in the circle of radius 4 at z = 4.
//!
//! The edit is the kernel's: `defeature::deleted` runs the faces round the deleted ones on along
//! their own surfaces and closes the gap they part off.
use crate::sketch::{block, cylinder, deleting, faces_through};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::cone;
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let cutter = combined(&cylinder([0., 0., 5.], 4., 6.)?,
        &cone([0., 0., 4.], [0., 0., 1.], 3., 4., 1.)?, Op::Union, 0.)?;
    let b = combined(&block([-10., -10., 0.], [10., 10., 10.])?, &cutter, Op::Cut, 0.)?;
    let bevel = faces_through(&b, &[[3.5, 0., 4.5]])?;
    deleting(&b, &bevel)
}
