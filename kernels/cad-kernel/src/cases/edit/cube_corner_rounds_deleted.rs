//! The cube from the origin to (10, 10, 10), the three edges meeting at its corner (10, 10, 10)
//! rounded to radius 2, three quarter cylinders and the eighth of a ball of radius 2 about (8, 8,
//! 8) where they meet; all four then deleted, the faces x = 10, y = 10 and z = 10 running on until
//! they meet in the three edges and the corner.
//!
//! The edit is the kernel's: `defeature::deleted` runs the faces round the deleted ones on along
//! their own surfaces and closes the gap they part off.
use crate::sketch::{block, deleting, faces_on};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::geom::{tolerance, Surface};
use cad_kernel::rounds::rounded;
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    // the cube's three edges at (10, 10, 10) rounded and the corner balled: the cube kept within a
    // box reaching past its other faces, every edge of that box rounded to 2 (a workaround:
    // `rounds::rounded` refuses a corner whose other edges stay sharp)
    let reach = block([-5., -5., -5.], [10., 10., 10.])?;
    let every: Vec<usize> = (0..reach.edges.len()).collect();
    let reach = rounded(&reach, &every, 2., tolerance(reach.size()))?;
    let b = combined(&block([0., 0., 0.], [10., 10., 10.])?, &reach, Op::Common, 0.)?;
    // the rounds are the faces that are not flat: three cylinders and the ball
    let rounds = faces_on(&b, |s| !matches!(s, Surface::Plane(..)));
    deleting(&b, &rounds)
}
