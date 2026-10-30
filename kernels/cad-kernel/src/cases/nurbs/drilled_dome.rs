//! The block under the Bézier dome of `nurbs/bezier-dome`, drilled straight down through by the
//! vertical cylinder of radius 3 about the line x = 12, y = 8.
use super::bezier_dome;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::topo::Brep;

const HOLE_AT: [f64; 2] = [12., 8.];
const HOLE_RADIUS: f64 = 3.;

pub fn build() -> Result<Brep, String> {
    let block = bezier_dome::build()?;
    // the drill from below the floor to above the dome's highest control point
    let [x, y] = HOLE_AT;
    let drill = prism(&disc([x, y, -5.], [0., 0., 1.], HOLE_RADIUS), 0., 2. * bezier_dome::SIDE)?;
    Ok(combined(&block, &drill, Op::Cut, 0.)?)
}
