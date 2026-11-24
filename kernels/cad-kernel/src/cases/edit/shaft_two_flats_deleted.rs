//! A shaft, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with two opposite flats
//! milled along its whole length, everything with |x| > 4 cut away, the strips of the planes x = 4
//! and x = −4 from y = −3 to y = 3; both flats then deleted together, the cylinder closing over
//! both and the ends growing back to whole discs.
//!
//! The edit is the kernel's: `defeature::deleted` runs the faces round the deleted ones on along
//! their own surfaces and closes the gap they part off.
use crate::sketch::{block, cylinder, deleting, faces_through};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let shaft = cylinder([0., 0., 0.], 5., 20.)?;
    let b = combined(&shaft, &block([4., -10., -1.], [10., 10., 21.])?, Op::Cut, 0.)?;
    let b = combined(&b, &block([-10., -10., -1.], [-4., 10., 21.])?, Op::Cut, 0.)?;
    let flats = faces_through(&b, &[[4., 0., 10.], [-4., 0., 10.]])?;
    deleting(&b, &flats)
}
