//! A plate from the origin to (40, 30, 6), all twelve of its edges rounded to radius 3, half its
//! thickness, in one call: each side's top and bottom rounds meet on its middle and the side is
//! gone, and each corner's two balls are one, so the upright edges keep no round at all.
use crate::sketch::block;
use cad_kernel::geom::tolerance;
use cad_kernel::rounds::rounded;
use cad_kernel::topo::Brep;

const ROUND: f64 = 3.;

pub fn build() -> Result<Brep, String> {
    let plate = block([0.; 3], [40., 30., 6.])?;
    // every edge, rounded together: the kernel puts a cylinder along each edge and a ball at each
    // corner, and where two rounds' contacts close a face to nothing, or a round's two corner
    // balls are one, it leaves that face or round out
    let edges: Vec<usize> = (0..plate.edges.len()).collect();
    rounded(&plate, &edges, ROUND, tolerance(plate.size()))
}
