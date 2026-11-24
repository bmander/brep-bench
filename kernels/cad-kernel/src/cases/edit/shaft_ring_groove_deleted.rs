//! A shaft, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a ring groove
//! turned into it, everything between radius 4 and 5 from z = 8 to z = 12 cut away: a floor, the
//! cylinder of radius 4, and two walls, the flat rings at z = 8 and z = 12; the floor and both
//! walls then deleted together, the shaft's cylinder below and above the groove, one surface,
//! running on across it until it is one face again.
//!
//! The edit is the kernel's: `defeature::deleted` runs the faces round the deleted ones on along
//! their own surfaces and closes the gap they part off.
use crate::sketch::{deleting, faces_through};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let shaft = prism(&disc([0., 0., 0.], [0., 0., 1.], 5.), 0., 20.)?;
    let ring = combined(&prism(&disc([0., 0., 8.], [0., 0., 1.], 7.), 0., 4.)?,
        &prism(&disc([0., 0., 7.], [0., 0., 1.], 4.), 0., 6.)?, Op::Cut, 0.)?;
    let b = combined(&shaft, &ring, Op::Cut, 0.)?;
    // the floor and the two walls
    let groove = faces_through(&b, &[[4., 0., 10.], [4.5, 0., 8.], [4.5, 0., 12.]])?;
    deleting(&b, &groove)
}
