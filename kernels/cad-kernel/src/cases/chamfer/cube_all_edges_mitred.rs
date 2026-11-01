//! A cube of side 10 at the origin with all twelve edges chamfered 2 on both faces, one after
//! another: each chamfer runs on until it meets the ones before, the three at each corner meeting
//! in a point.
use crate::sketch::{block, named};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::fillet::flat_along;
use cad_kernel::geom::tolerance;
use cad_kernel::topo::Brep;

const SIDE: f64 = 10.;
const LEG: f64 = 2.;

pub fn build() -> Result<Brep, String> {
    let mut cube = block([0., 0., 0.], [SIDE; 3])?;
    // each edge picked by its middle: the four along x, then y, then z
    let h = SIDE / 2.;
    let mut edges = Vec::new();
    for axis in 0..3 {
        for a in [0., SIDE] {
            for b in [0., SIDE] {
                let mut at = [0.; 3];
                (at[axis], at[(axis + 1) % 3], at[(axis + 2) % 3]) = (h, a, b);
                edges.push(at);
            }
        }
    }
    for at in edges {
        cube = named(cube);
        let flat = flat_along(&cube, at, [LEG, LEG], None, tolerance(cube.size()))?;
        cube = combined(&cube, &flat, Op::Cut, 0.)?;
    }
    Ok(cube)
}
