//! A half cylinder — the half of the cylinder of radius 5 on the z axis from z = 0 to z = 6 where
//! x ≥ 0 — its curved rim, the half circle where its side meets its top, rounded with a radius
//! that grows linearly with the angle φ round the z axis, r = 3/2 + φ/π: 1 at (0, −5, 6), 1.5 at
//! (5, 0, 6) and 2 at (0, 5, 6).
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, kept, prism};
use cad_kernel::fillet::roll_varying;
use cad_kernel::geom::tolerance;
use cad_kernel::topo::Brep;
use std::f64::consts::PI;

const R: f64 = 5.;
const HEIGHT: f64 = 6.;

pub fn build() -> Result<Brep, String> {
    // the cylinder, kept where x ≥ 0
    let cylinder = prism(&disc([0.; 3], [0., 0., 1.], R), 0., HEIGHT)?;
    let half = kept(&cylinder, [0.; 3], [1., 0., 0.], 0.)?;
    // the rim picked by its middle; the kernel rolls, at each point of it, the ball of the radius
    // there touching the side and the top with its centre in the plane square to the rim, and cuts
    // the round off where it meets the flat face
    let radius = |[x, y, _]: [f64; 3]| 1.5 + y.atan2(x) / PI;
    let round = roll_varying(&half, [R * 0.3f64.cos(), R * 0.3f64.sin(), HEIGHT], &radius, tolerance(half.size()))?;
    let op = if round.concave { Op::Union } else { Op::Cut };
    Ok(combined(&half, &round.piece, op, 0.)?)
}
