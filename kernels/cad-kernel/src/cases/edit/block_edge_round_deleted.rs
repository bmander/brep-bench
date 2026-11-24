//! The box from the origin to (20, 10, 10), the edge where its top, z = 10, meets its side x = 20
//! rounded to radius 2; the round then deleted, the top and the side run on until they meet in the
//! edge again.
//!
//! The edit is the kernel's: `defeature::deleted` runs the faces round the deleted ones on along
//! their own surfaces and closes the gap they part off.
use crate::sketch::{block, deleting, faces_on, named, rounded_edge};
use cad_kernel::geom::Surface;
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let b = rounded_edge(&named(block([0., 0., 0.], [20., 10., 10.])?), [20., 5., 10.], 2.)?;
    let round = faces_on(&b, &|s| matches!(s, Surface::Cylinder(..)));
    deleting(&b, &round)
}
