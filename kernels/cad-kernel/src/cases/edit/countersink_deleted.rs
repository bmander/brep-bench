//! The box from the origin to (20, 10, 10), drilled right through along z by a hole of radius 2
//! about the line through (10, 5) and countersunk at the top by the cone from radius 2 at z = 8 to
//! radius 4 at z = 10; the countersink's cone then deleted, the hole's wall running on up from
//! z = 8 and the top closing in from radius 4 until they meet in the circle of radius 2 at z = 10.
//!
//! The edit is the kernel's: `defeature::deleted` runs the faces round the deleted ones on along
//! their own surfaces and closes the gap they part off.
use crate::sketch::{block, cylinder, deleting, faces_through};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::cone;
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let drill = cylinder([10., 5., -1.], 2., 12.)?;
    let b = combined(&block([0., 0., 0.], [20., 10., 10.])?, &drill, Op::Cut, 0.)?;
    // the countersink's cone, from radius 1 at z = 7 to 5 at z = 11: past the top, and meeting the
    // hole's wall at z = 8
    let b = combined(&b, &cone([10., 5., 7.], [0., 0., 1.], 1., 5., 4.)?, Op::Cut, 0.)?;
    let sink = faces_through(&b, &[[13., 5., 9.]])?;
    deleting(&b, &sink)
}
