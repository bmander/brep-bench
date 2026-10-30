//! A cylinder of radius 5 on z from z = 0 to 10, the rim of its top face chamfered 5 on both faces,
//! its own radius: the top shrinks to its centre and the chamfer is a cone.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::fillet::flat_along;
use cad_kernel::geom::tolerance;
use cad_kernel::topo::Brep;

const RADIUS: f64 = 5.;
const HEIGHT: f64 = 10.;
const DISTANCE: f64 = RADIUS;

pub fn build() -> Result<Brep, String> {
    // the disc swept up, its circle's face named `side` (the caps are `far` and `near`)
    let mut section = disc([0., 0., 0.], [0., 0., 1.], RADIUS);
    section.names = vec![vec!["side".into()]];
    let cylinder = prism(&section, 0., HEIGHT)?;
    // the rim is picked by a point on it; the kernel cuts the flat between the leg 5 in across the
    // top (from the side's surface) and the leg 5 down the side (from the top's), and builds the
    // ring of material between it and the rim
    let legs = [DISTANCE, DISTANCE];
    let at = [0., RADIUS, HEIGHT];
    let piece = flat_along(&cylinder, at, legs, None, tolerance(cylinder.size()))?;
    // the rim is convex: its piece is taken away
    Ok(combined(&cylinder, &piece, Op::Cut, 0.)?)
}
