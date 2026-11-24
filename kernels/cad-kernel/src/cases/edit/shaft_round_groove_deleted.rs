//! A shaft, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a round-bottomed
//! groove turned into it, the torus whose tube of radius 1.5 runs round the circle of radius 5 at z
//! = 10 cut away: one face, the torus's inner half from z = 8.5 to z = 11.5; that face then
//! deleted, the shaft's cylinder below and above the groove, one surface, running on across it
//! until it is one face again.
//!
//! The edit is the kernel's: `defeature::deleted` runs the faces round the deleted ones on along
//! their own surfaces and closes the gap they part off.
use crate::sketch::{deleting, faces_through};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism, torus};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let shaft = prism(&disc([0., 0., 0.], [0., 0., 1.], 5.), 0., 20.)?;
    let b = combined(&shaft, &torus([0., 0., 10.], [0., 0., 1.], 5., 1.5)?, Op::Cut, 0.)?;
    let groove = faces_through(&b, &[[3.5, 0., 10.]])?;
    deleting(&b, &groove)
}
