//! A tube about the z axis from z = 0 to z = 20, outer radius 10 and bore of radius 4, with a ring
//! groove turned into the bore, everything inside radius 6 from z = 8 to z = 12 cut away: a floor,
//! the cylinder of radius 6, and two walls, the flat rings from radius 4 to 6 at z = 8 and z = 12;
//! the floor and both walls then deleted together, the bore below and above the groove, one
//! surface, running on across it until it is one face again.
//!
//! The edit is the kernel's: `defeature::deleted` runs the faces round the deleted ones on along
//! their own surfaces and closes the gap they part off.
use crate::sketch::{cylinder, deleting, faces_through};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let tube = combined(&cylinder([0., 0., 0.], 10., 20.)?,
        &cylinder([0., 0., -1.], 4., 22.)?, Op::Cut, 0.)?;
    let b = combined(&tube, &cylinder([0., 0., 8.], 6., 4.)?, Op::Cut, 0.)?;
    // the floor and the two walls
    let groove = faces_through(&b, &[[6., 0., 10.], [5., 0., 8.], [5., 0., 12.]])?;
    deleting(&b, &groove)
}
