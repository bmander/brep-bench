//! A regular hexagonal prism from z = 0 to z = 10 on the hexagon of side 6 centred on the z axis,
//! its corners at (6 cos 60k°, 6 sin 60k°); its edge from (6, 0, 0) to (6, 0, 10), where its sides
//! meet at 120°, rounded with a radius that grows linearly along it, r = 1 + 3z/10.
//!
//! The kernel rolls it (`fillet::roll_varying`): at each point of the edge the ball of the radius
//! there touching both sides, its centre in the plane square to the edge, and cuts the round off
//! where it meets the top and the bottom.
use crate::sketch::polygon;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::prism;
use cad_kernel::fillet::roll_varying;
use cad_kernel::geom::tolerance;
use cad_kernel::topo::Brep;
use std::f64::consts::PI;

const SIDE: f64 = 6.;
const HEIGHT: f64 = 10.;

pub fn build() -> Result<Brep, String> {
    let corners: Vec<[f64; 3]> = (0..6)
        .map(|k| { let a = k as f64 * PI / 3.; [SIDE * a.cos(), SIDE * a.sin(), 0.] }).collect();
    let body = prism(&polygon(&corners), 0., HEIGHT)?;
    // the edge at (6, 0) picked by its middle, its radius r = 1 + 3z/10 at each of its points
    let round = roll_varying(&body, [SIDE, 0., HEIGHT / 2.], &|p| 1. + 0.3 * p[2], tolerance(body.size()))?;
    let op = if round.concave { Op::Union } else { Op::Cut };
    Ok(combined(&body, &round.piece, op, 0.)?)
}
