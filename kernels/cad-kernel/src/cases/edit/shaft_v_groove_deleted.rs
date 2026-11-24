//! A shaft, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a V-groove turned
//! into it, everything farther from the axis than 3 + |z − 10| cut away: two cones meeting at
//! radius 3 at z = 10, running out to the shaft at z = 8 and z = 12; both cones then deleted
//! together, the shaft's cylinder below and above the groove, one surface, running on across it
//! until it is one face again.
//!
//! The edit is the kernel's: `defeature::deleted` runs the faces round the deleted ones on along
//! their own surfaces and closes the gap they part off.
use crate::sketch::{deleting, faces_through, turned};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let b = turned(&[[5., 0.], [5., 8.], [3., 10.], [5., 12.], [5., 20.]])?;
    let groove = faces_through(&b, &[[4., 0., 9.], [4., 0., 11.]])?;
    deleting(&b, &groove)
}
