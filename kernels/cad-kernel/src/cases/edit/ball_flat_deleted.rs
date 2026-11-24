//! The ball of radius 5 about the origin with its top cut flat, everything above z = 3 cut away,
//! leaving a flat disc of radius 4 at z = 3; the flat then deleted, the ball closing over the
//! circle of radius 4 its edge is.
//!
//! The edit is the kernel's: `defeature::deleted` runs the faces round the deleted ones on along
//! their own surfaces and closes the gap they part off.
use crate::sketch::{ball_below, deleting, faces_through};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let b = ball_below([0., 0., 0.], 5., 3.)?;
    let flat = faces_through(&b, &[[1., 0., 3.]])?;
    deleting(&b, &flat)
}
