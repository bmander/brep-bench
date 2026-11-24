//! The box from the origin to (20, 16, 12) with every edge rounded to radius 2, twelve quarter
//! cylinders along its edges and eight eighths of a ball at its corners; all twenty rounds then
//! deleted, the six flat faces running on until they meet in the box's twelve edges and eight
//! corners.
//!
//! The edit is the kernel's: `defeature::deleted` runs the faces round the deleted ones on along
//! their own surfaces and closes the gap they part off.
use crate::sketch::{block, deleting, faces_on};
use cad_kernel::geom::{tolerance, Surface};
use cad_kernel::rounds::rounded;
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let b = block([0., 0., 0.], [20., 16., 12.])?;
    let every: Vec<usize> = (0..b.edges.len()).collect();
    let b = rounded(&b, &every, 2., tolerance(b.size()))?;
    // the rounds are every face that is not flat: twelve cylinders and eight balls
    let rounds = faces_on(&b, |s| !matches!(s, Surface::Plane(..)));
    deleting(&b, &rounds)
}
