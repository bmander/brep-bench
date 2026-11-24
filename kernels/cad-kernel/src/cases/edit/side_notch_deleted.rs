//! The box from the origin to (20, 10, 10) with a half-round notch in its side y = 0, the cylinder
//! of radius 3 about the line through (10, 0) cut away top to bottom; the notch's half cylinder
//! then deleted, the side running on from x = 7 and from x = 13 until it is one rectangle again,
//! and the top and the bottom closing over the half disks the notch took.
//!
//! The edit is the kernel's: `defeature::deleted` runs the faces round the deleted ones on along
//! their own surfaces and closes the gap they part off.
use crate::sketch::{block, cylinder, deleting, faces_through};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let notch = cylinder([10., 0., -1.], 3., 12.)?;
    let b = combined(&block([0., 0., 0.], [20., 10., 10.])?, &notch, Op::Cut, 0.)?;
    let wall = faces_through(&b, &[[10., 3., 5.]])?;
    deleting(&b, &wall)
}
