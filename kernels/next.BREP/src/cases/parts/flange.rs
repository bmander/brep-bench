//! A round mounting flange on the z axis: a plate, the disc of radius 32 from z = 0 to z = 6, and
//! a hub, the cylinder of radius 15 from z = 6 to z = 16, joined; a bore of radius 8 through both
//! on the axis; and six bolt holes of radius 3 through the plate, their axes on the circle of
//! radius 24 one every 60° from (24, 0), each cut by its own cylinder running past both faces.
use crate::solids::fillet::{on_circle, subtract, union};
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_cylinder_brep, BrepSolid};
use std::f64::consts::FRAC_PI_3;

pub fn build() -> Result<BrepSolid, Refusal> {
    let up = p(0.0, 0.0, 1.0);
    let plate = make_cylinder_brep(p(0.0, 0.0, 0.0), up, 32.0, 6.0)?;
    let hub = make_cylinder_brep(p(0.0, 0.0, 6.0), up, 15.0, 10.0)?;
    let bore = make_cylinder_brep(p(0.0, 0.0, -1.0), up, 8.0, 18.0)?;
    let mut flange = subtract(&union(&plate, &hub)?, &bore)?;
    for k in 0..6 {
        let hole = make_cylinder_brep(on_circle(24.0, -1.0, FRAC_PI_3 * k as f64), up, 3.0, 8.0)?;
        flange = subtract(&flange, &hole)?;
    }
    Ok(flange)
}
