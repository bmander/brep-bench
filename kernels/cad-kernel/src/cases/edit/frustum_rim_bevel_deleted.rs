//! The frustum on the z axis from radius 6 at z = 0 to radius 3.5 at z = 5, its side the cone
//! 2r + z = 12, joined to the steeper frustum from radius 3.5 at z = 5 to radius 2 at z = 6, which
//! bevels its top rim; the bevel's cone then deleted, the top running on outward from radius 2 and
//! the side up from z = 5 until they meet in the circle of radius 3 at z = 6.
//!
//! The edit is the kernel's: `defeature::deleted` runs the faces round the deleted ones on along
//! their own surfaces and closes the gap they part off.
use crate::sketch::{deleting, faces_through, turned};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let b = turned(&[[6., 0.], [3.5, 5.], [2., 6.]])?;
    let bevel = faces_through(&b, &[[2.75, 0., 5.5]])?;
    deleting(&b, &bevel)
}
