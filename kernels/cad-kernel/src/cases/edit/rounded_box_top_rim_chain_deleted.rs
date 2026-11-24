//! The box from the origin to (20, 12, 8), its four upright edges rounded to radius 3, then the
//! whole rim of its top rounded to radius 1, four quarter cylinders and four pieces of tori running
//! into one another in a closed chain; the eight faces of the chain then deleted together, the top
//! running on outward and the walls up until they meet in the top's rim again.
//!
//! The edit is the kernel's: `defeature::deleted` runs the faces round the deleted ones on along
//! their own surfaces and closes the gap they part off.
use crate::sketch::{deleting, faces_on, named, rounded_edge, rounded_plate};
use cad_kernel::geom::Surface;
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let b = named(rounded_plate([0., 0.], [20., 12.], 3., 8.)?);
    // the top's rim rolled as one chain from a point on its long side
    let b = rounded_edge(&b, [10., 0., 8.], 1.)?;
    // the chain: every face not flat but the corners' upright cylinders, of radius 3
    let chain = faces_on(&b, &|s| !matches!(s, Surface::Plane(..)) && !matches!(s, Surface::Cylinder(_, r) if *r > 2.));
    deleting(&b, &chain)
}
