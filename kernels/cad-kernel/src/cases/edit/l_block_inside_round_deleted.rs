//! The L-shaped block through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15), 8 high, its
//! inside corner along x = 5, y = 5 rounded to radius 2 by the quarter cylinder about the line
//! x = 7, y = 7; the round then deleted, the face x = 5 running back from y = 7 and the face y = 5
//! from x = 7 until they meet in the edge x = 5, y = 5.
//!
//! The edit is the kernel's: `defeature::deleted` runs the faces round the deleted ones on along
//! their own surfaces and closes the gap they part off.
use crate::sketch::{deleting, faces_on, l_block, named, rounded_edge};
use cad_kernel::geom::Surface;
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let b = rounded_edge(&named(l_block()?), [5., 5., 4.], 2.)?;
    let round = faces_on(&b, |s| matches!(s, Surface::Cylinder(..)));
    deleting(&b, &round)
}
