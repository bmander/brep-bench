//! A plate, the cylinder of radius 8 on the z axis from z = −4 to z = 0, with a boss on it, the
//! cylinder of radius 3 from z = 0 to z = 8, the concave edge at the boss's foot rounded to radius
//! 1 by a quarter of the torus whose tube has radius 1 about the circle of radius 4 at z = 1; the
//! round then deleted, the plate's top running in from radius 4 and the boss's side down from z = 1
//! until they meet in the circle of radius 3 at z = 0.
//!
//! The edit is the kernel's: `defeature::deleted` runs the faces round the deleted ones on along
//! their own surfaces and closes the gap they part off.
use crate::sketch::{deleting, faces_on, named, rounded_edge};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::geom::Surface;
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let plate = prism(&disc([0., 0., -4.], [0., 0., 1.], 8.), 0., 4.)?;
    let b = named(combined(&plate, &prism(&disc([0., 0., 0.], [0., 0., 1.], 3.), 0., 8.)?, Op::Union, 0.)?);
    let b = rounded_edge(&b, [3., 0., 0.], 1.)?;
    let round = faces_on(&b, &|s| matches!(s, Surface::Torus(..)));
    deleting(&b, &round)
}
