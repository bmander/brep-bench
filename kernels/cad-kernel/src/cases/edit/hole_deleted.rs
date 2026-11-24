//! The box from the origin to (20, 10, 10), drilled right through along z by a hole of radius 3
//! about the line through (10, 5); the hole's wall then deleted, the top and the bottom closing
//! over the circles its two ends are.
//!
//! The edit is the kernel's: `defeature::deleted` runs the faces round the deleted ones on along
//! their own surfaces and closes the gap they part off.
use crate::sketch::{block, deleting, faces_through};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let drill = prism(&disc([10., 5., -1.], [0., 0., 1.], 3.), 0., 12.)?;
    let b = combined(&block([0., 0., 0.], [20., 10., 10.])?, &drill, Op::Cut, 0.)?;
    let wall = faces_through(&b, &[[13., 5., 5.]])?;
    deleting(&b, &wall)
}
