//! The box from the origin to (20, 10, 10), the two long edges of its top rounded to radius 2 by
//! the quarter cylinders about the lines y = 2, z = 8 and y = 8, z = 8; both rounds then deleted
//! together, the top running on from y = 2 and from y = 8 and the sides y = 0 and y = 10 up from
//! z = 8 until they meet in the edges at z = 10, and the ends x = 0 and x = 20 getting their
//! corners back.
//!
//! The edit is the kernel's: `defeature::deleted` runs the faces round the deleted ones on along
//! their own surfaces and closes the gap they part off.
use crate::sketch::{block, deleting, faces_on, named, rounded_edge};
use cad_kernel::geom::Surface;
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let b = named(block([0., 0., 0.], [20., 10., 10.])?);
    let b = rounded_edge(&b, [10., 0., 10.], 2.)?;
    let b = rounded_edge(&b, [10., 10., 10.], 2.)?;
    // the rounds are the block's two cylinders
    let rounds = faces_on(&b, &|s| matches!(s, Surface::Cylinder(..)));
    deleting(&b, &rounds)
}
