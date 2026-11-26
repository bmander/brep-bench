//! An L-shaped block 10 deep — the L through (0, 0), (20, 0), (20, 4), (6, 4), (6, 12), (0, 12), as
//! (x, z), extruded from y = 0 to y = 10 — its one inside edge, from (6, 0, 4) to (6, 10, 4),
//! rounded with a radius that grows linearly along it, r = 1 + y/5, filling the inside corner.
//!
//! The kernel rolls it (`fillet::roll_varying`): at each point of the edge the ball of the radius
//! there touching both faces, its centre in the plane square to the edge, and fills the corner
//! between them, cut off where it meets the two ends.
use crate::sketch::polygon;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::prism;
use cad_kernel::fillet::roll_varying;
use cad_kernel::geom::tolerance;
use cad_kernel::topo::Brep;

const DEPTH: f64 = 10.;

pub fn build() -> Result<Brep, String> {
    // the L in the xz plane, swept along +y (walked this way round its profile faces +y)
    let l = polygon(&[[0., 0., 0.], [0., 0., 12.], [6., 0., 12.], [6., 0., 4.], [20., 0., 4.],
                      [20., 0., 0.]]);
    let body = prism(&l, 0., DEPTH)?;
    // the inside edge picked by its middle, its radius r = 1 + y/5 at each of its points
    let round = roll_varying(&body, [6., DEPTH / 2., 4.], &|p| 1. + p[1] / 5., tolerance(body.size()))?;
    let op = if round.concave { Op::Union } else { Op::Cut };
    Ok(combined(&body, &round.piece, op, 0.)?)
}
