//! The cone frustum about the z axis of radius 6 at z = 0 narrowing to radius 3 at z = 20, a flat
//! milled along its side, everything with y < −4 cut away, the flat running up from the base to
//! where the cone narrows past it, its edge on the cone a hyperbola; the flat then deleted, the
//! cone closing over it and the base growing back to a whole disc.
//!
//! The edit is the kernel's: `defeature::deleted` runs the faces round the deleted ones on along
//! their own surfaces and closes the gap they part off.
use crate::sketch::{block, deleting, faces_through};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::cone;
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let frustum = cone([0., 0., 0.], [0., 0., 1.], 6., 3., 20.)?;
    let b = combined(&frustum, &block([-10., -10., -1.], [10., -4., 21.])?, Op::Cut, 0.)?;
    let flat = faces_through(&b, &[[0., -4., 1.]])?;
    deleting(&b, &flat)
}
