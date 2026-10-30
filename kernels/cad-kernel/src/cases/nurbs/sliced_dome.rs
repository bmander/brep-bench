//! The block under the Bézier dome of `nurbs/bezier-dome`, with everything above the plane
//! z = 4 + x/2 cut away: a plane through the block's corner (0, 0, 4).
use super::bezier_dome;
use cad_kernel::build::kept;
use cad_kernel::topo::Brep;
use cad_kernel::util::space::unit;

const CORNER: [f64; 3] = [0., 0., 4.];
const SLOPE: f64 = 0.5; // dz/dx of the plane

pub fn build() -> Result<Brep, String> {
    let block = bezier_dome::build()?;
    // keep the side of the plane below it: against its upward normal (−½, 0, 1)
    kept(&block, CORNER, unit([SLOPE, 0., -1.]), 0.)
}
