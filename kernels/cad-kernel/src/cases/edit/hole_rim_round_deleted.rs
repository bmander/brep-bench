//! The box from the origin to (20, 10, 10), drilled through along z by a hole of radius 2 about the
//! line x = 10, y = 5, the edge where the hole's wall meets the top rounded to radius 1 by a
//! quarter of the torus whose tube has radius 1 about the circle of radius 3 at z = 9; the round
//! then deleted, the top running on inward from radius 3 and the wall up from z = 9 until they meet
//! in the circle of radius 2 at z = 10.
//!
//! The edit is the kernel's: `defeature::deleted` runs the faces round the deleted ones on along
//! their own surfaces and closes the gap they part off.
use crate::sketch::{block, deleting, faces_on, named, rounded_edge};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::geom::Surface;
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let drill = prism(&disc([10., 5., -1.], [0., 0., 1.], 2.), 0., 12.)?;
    let b = named(combined(&block([0., 0., 0.], [20., 10., 10.])?, &drill, Op::Cut, 0.)?);
    let b = rounded_edge(&b, [12., 5., 10.], 1.)?;
    let round = faces_on(&b, &|s| matches!(s, Surface::Torus(..)));
    deleting(&b, &round)
}
