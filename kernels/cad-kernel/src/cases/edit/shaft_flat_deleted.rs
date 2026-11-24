//! A shaft, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a flat milled
//! along its whole length, everything with y < −4 cut away; the flat, the strip of the plane y = −4
//! from x = −3 to x = 3, then deleted, the cylinder closing over it and the ends growing back to
//! whole discs.
//!
//! The edit is the kernel's: `defeature::deleted` runs the faces round the deleted ones on along
//! their own surfaces and closes the gap they part off.
use crate::sketch::{block, cylinder, deleting, faces_through};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let shaft = cylinder([0., 0., 0.], 5., 20.)?;
    let b = combined(&shaft, &block([-10., -10., -1.], [10., -4., 21.])?, Op::Cut, 0.)?;
    let flat = faces_through(&b, &[[0., -4., 10.]])?;
    deleting(&b, &flat)
}
