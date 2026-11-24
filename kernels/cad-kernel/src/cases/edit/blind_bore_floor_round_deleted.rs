//! The box from (−10, −10, 0) to (10, 10, 10) with a blind bore of radius 4 about the z axis sunk
//! from its top to a flat floor at z = 4, the concave edge where the floor meets the wall rounded
//! to radius 1 by a quarter of the torus whose tube has radius 1 about the circle of radius 3 at
//! z = 5; the round then deleted, the floor running on outward from radius 3 and the wall down from
//! z = 5 until they meet in the circle of radius 4 at z = 4.
//!
//! The edit is the kernel's: `defeature::deleted` runs the faces round the deleted ones on along
//! their own surfaces and closes the gap they part off.
use crate::sketch::{block, deleting, faces_on, named, rounded_edge};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::geom::Surface;
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let bore = prism(&disc([0., 0., 4.], [0., 0., 1.], 4.), 0., 7.)?;
    let b = named(combined(&block([-10., -10., 0.], [10., 10., 10.])?, &bore, Op::Cut, 0.)?);
    let b = rounded_edge(&b, [4., 0., 4.], 1.)?;
    let round = faces_on(&b, &|s| matches!(s, Surface::Torus(..)));
    deleting(&b, &round)
}
