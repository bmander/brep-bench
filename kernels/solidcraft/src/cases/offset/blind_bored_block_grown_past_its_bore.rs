//! The block from (−10, −10, 0) to (10, 10, 10) with a blind bore of radius 3 down its z axis from
//! the top to a floor at z = 2, grown by 4, more than the bore's radius: every point within 4 of
//! it, the kernel's offset with its convex edges rounded.
use crate::bench::cut;
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::{make_box, make_cylinder, Body};

pub fn build() -> Result<Body, Refusal> {
    let block = make_box([-10.0, -10.0, 0.0], [20.0, 20.0, 10.0])?;
    // the bore runs on past the top
    let bore = make_cylinder([0.0, 0.0, 2.0], 3.0, 9.0)?;
    offset_rounded(&cut(block, bore)?, 4.0)
}
