//! A hub, the tube about the z axis from z = 0 to z = 20 of outer radius 10 and bore of radius 4,
//! with a keyway cut along the bore's whole length, the box −1 ≤ x ≤ 1, −5 ≤ y ≤ 0 cut away: its
//! floor y = −5 and its walls x = −1 and x = 1; the keyway's three faces then deleted, the bore
//! closing over it and the hub's ends growing back to whole rings.
//!
//! The edit is the kernel's: `defeature::deleted` runs the faces round the deleted ones on along
//! their own surfaces and closes the gap they part off.
use crate::sketch::{block, cylinder, deleting, faces_through};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let hub = combined(&cylinder([0., 0., 0.], 10., 20.)?,
        &cylinder([0., 0., -1.], 4., 22.)?, Op::Cut, 0.)?;
    let b = combined(&hub, &block([-1., -5., -1.], [1., 0., 21.])?, Op::Cut, 0.)?;
    // the floor and the two walls
    let keyway = faces_through(&b, &[[0., -5., 10.], [1., -4.5, 10.], [-1., -4.5, 10.]])?;
    deleting(&b, &keyway)
}
