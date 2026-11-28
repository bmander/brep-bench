//! The plate from (0, 0, 0) to (10, 10, 2) with the square hole from (3, 3) to (7, 7) through it,
//! grown by 1: every point within 1 of it, its convex edges rounded and the hole's four concave
//! corners sharp, the kernel's offset with its convex edges rounded.
use crate::bench::cut;
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::{make_box, Body};

pub fn build() -> Result<Body, Refusal> {
    let plate = make_box([0.0, 0.0, 0.0], [10.0, 10.0, 2.0])?;
    // the hole's cutter runs past both faces
    let hole = make_box([3.0, 3.0, -1.0], [4.0, 4.0, 4.0])?;
    offset_rounded(&cut(plate, hole)?, 1.0)
}
