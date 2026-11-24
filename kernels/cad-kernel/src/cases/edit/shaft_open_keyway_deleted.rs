//! A shaft, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with an open keyway cut
//! into it, the box −1 ≤ x ≤ 1, y ≤ −3, z ≥ 8 cut away, running from z = 8 out through the top: its
//! floor y = −3, its walls x = −1 and x = 1 and its end z = 8; all four then deleted, the cylinder
//! closing over the keyway and the top growing back to a whole disc.
//!
//! The edit is the kernel's: `defeature::deleted` runs the faces round the deleted ones on along
//! their own surfaces and closes the gap they part off.
use crate::sketch::{block, cylinder, deleting, faces_through};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let shaft = cylinder([0., 0., 0.], 5., 20.)?;
    let b = combined(&shaft, &block([-1., -10., 8.], [1., -3., 21.])?, Op::Cut, 0.)?;
    // the floor, the two walls and the end
    let keyway = faces_through(&b, &[[0., -3., 15.], [1., -4., 15.], [-1., -4., 15.], [0., -4., 8.]])?;
    deleting(&b, &keyway)
}
