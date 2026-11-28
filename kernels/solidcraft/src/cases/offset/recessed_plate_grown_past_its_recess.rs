//! The plate from (−10, −10, 0) to (10, 10, 4) with a round recess of radius 3 sunk 1 into its top
//! about the z axis, grown by 4, more than the recess's radius: every point within 4 of it, the
//! kernel's offset with its convex edges rounded.
use crate::bench::cut;
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::{make_box, make_cylinder, Body};

pub fn build() -> Result<Body, Refusal> {
    let plate = make_box([-10.0, -10.0, 0.0], [20.0, 20.0, 4.0])?;
    // the recess's cutter runs on past the top
    let recess = make_cylinder([0.0, 0.0, 3.0], 3.0, 2.0)?;
    offset_rounded(&cut(plate, recess)?, 4.0)
}
