//! The block from (0, 0, 0) to (20, 20, 6) with the square pocket from (6, 6) to (14, 14) cut down
//! from its top to a floor at z = 3, grown by 1: every point within 1 of it, its convex edges
//! rounded and the pocket's concave ones sharp, the kernel's offset with its convex edges rounded.
use crate::bench::cut;
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::{make_box, Body};

pub fn build() -> Result<Body, Refusal> {
    let block = make_box([0.0, 0.0, 0.0], [20.0, 20.0, 6.0])?;
    // the pocket's cutter runs on past the top
    let pocket = make_box([6.0, 6.0, 3.0], [8.0, 8.0, 4.0])?;
    offset_rounded(&cut(block, pocket)?, 1.0)
}
