//! A plate, the disk of radius 8 about the z axis from z = −2 to z = 0, joined to the ball of
//! radius 3 about (0, 0, 1), the groove where they meet filled by a bevel, the frustum from radius
//! 4 at z = 0 to radius 3 at z = 1; the bevel's cone then deleted, the plate's top running on
//! inward and the ball down from its equator until they meet in the circle of radius √8 at z = 0.
//!
//! The edit is the kernel's: `defeature::deleted` runs the faces round the deleted ones on along
//! their own surfaces and closes the gap they part off.
use crate::sketch::{deleting, faces_through};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{cone, disc, prism, sphere};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let plate = prism(&disc([0., 0., -2.], [0., 0., 1.], 8.), 0., 2.)?;
    // the ball's axis along x, so neither pole lies on the plate's faces
    let b = combined(&plate, &sphere([0., 0., 1.], [1., 0., 0.], 3.)?, Op::Union, 0.)?;
    let b = combined(&b, &cone([0., 0., 0.], [0., 0., 1.], 4., 3., 1.)?, Op::Union, 0.)?;
    let bevel = faces_through(&b, &[[3.5, 0., 0.5]])?;
    deleting(&b, &bevel)
}
