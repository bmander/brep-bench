//! The plate from (−10, −10, 0) to (10, 10, 4) with a hole of radius 3 drilled through it along
//! the z axis, grown by 3, the hole's radius: every point within 3 of it. The plate grows as a box
//! does, the hole closes onto its axis, and over each end of it the grown face dips in a dimple,
//! the inside of a horn torus about the rim.
use crate::bench::cut;
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::{make_box, make_cylinder, Body};

pub fn build() -> Result<Body, Refusal> {
    grown(1.0)
}

/// The plate, its hole and the growth with every length `s` times as long.
pub fn grown(s: f64) -> Result<Body, Refusal> {
    let plate = make_box([-10.0 * s, -10.0 * s, 0.0], [20.0 * s, 20.0 * s, 4.0 * s])?;
    // the drill runs past both faces
    let drill = make_cylinder([0.0, 0.0, -1.0 * s], 3.0 * s, 6.0 * s)?;
    let drilled = cut(plate, drill)?;
    offset_rounded(&drilled, 3.0 * s)
}
