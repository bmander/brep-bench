//! The cylinder of radius 6 on the z axis from z = 0 to z = 8 with the frustum from radius 6 at
//! z = 8 to radius 4 at z = 10 on it, a cylinder to z = 10 whose top rim is beveled 2 back on each
//! face; the bevel, the frustum's side, then deleted, the top running on from radius 4 and the side
//! from z = 8 until they meet in the circle of radius 6 at z = 10.
//!
//! The edit is the kernel's: `defeature::deleted` runs the faces round the deleted ones on along
//! their own surfaces and closes the gap they part off.
use crate::sketch::{deleting, faces_on, turned};
use cad_kernel::geom::Surface;
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let b = turned(&[[6., 0.], [6., 8.], [4., 10.]])?;
    let bevel = faces_on(&b, &|s| matches!(s, Surface::Cone(..)));
    deleting(&b, &bevel)
}
