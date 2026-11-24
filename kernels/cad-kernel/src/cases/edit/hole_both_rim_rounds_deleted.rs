//! The box from the origin to (20, 10, 10), drilled through along z by a hole of radius 2 about the
//! line x = 10, y = 5, both of the hole's rims rounded to radius 1 by quarters of the tori whose
//! tubes have radius 1 about the circles of radius 3 at z = 9 and at z = 1; both rounds then
//! deleted together, the top and the bottom running on inward from radius 3 and the wall up from
//! z = 9 and down from z = 1 until they meet in the circles of radius 2 at z = 10 and at z = 0.
//!
//! The edit is the kernel's: `defeature::deleted` runs the faces round the deleted ones on along
//! their own surfaces and closes the gap they part off.
use crate::sketch::{block, cylinder, deleting, faces_on, named, rounded_edge};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::geom::Surface;
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let drill = cylinder([10., 5., -1.], 2., 12.)?;
    let b = named(combined(&block([0., 0., 0.], [20., 10., 10.])?, &drill, Op::Cut, 0.)?);
    let b = rounded_edge(&b, [12., 5., 10.], 1.)?;
    let b = rounded_edge(&b, [12., 5., 0.], 1.)?;
    let rounds = faces_on(&b, |s| matches!(s, Surface::Torus(..)));
    deleting(&b, &rounds)
}
