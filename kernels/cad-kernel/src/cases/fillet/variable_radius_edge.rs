//! A cube of side 10 between the origin and (10, 10, 10), its edge where the faces x = 10 and
//! y = 10 meet rounded with a radius growing linearly along it: r = 1 + z/10, 1 at the bottom face
//! and 2 at the top.
//!
//! The kernel rolls it (`fillet::roll_varying`): at each point of the edge the ball of the radius
//! there touching both faces, its centre in the plane square to the edge, and cuts the round off
//! where it meets the top and the bottom.
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::fillet::roll_varying;
use cad_kernel::geom::tolerance;
use cad_kernel::topo::Brep;

const SIDE: f64 = 10.;

pub fn build() -> Result<Brep, String> {
    let cube = block([0.; 3], [SIDE; 3])?;
    // the edge picked by its middle, its radius r = 1 + z/10 at each of its points
    let round = roll_varying(&cube, [SIDE, SIDE, SIDE / 2.], &|p| 1. + p[2] / 10., tolerance(cube.size()))?;
    let op = if round.concave { Op::Union } else { Op::Cut };
    Ok(combined(&cube, &round.piece, op, 0.)?)
}
