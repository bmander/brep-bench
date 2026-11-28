//! The plate from (−20, −15, 0) to (20, 15, 4) with a window cut through it, the rectangle from
//! (−10, −7) to (10, 7) with its corners rounded to radius 2, grown by 3, more than the corners'
//! radius: every point within 3 of it, the kernel's offset with its convex edges rounded.
use crate::bench::cut;
use crate::shared::draft::rounded_box;
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::{make_box, Body};

pub fn build() -> Result<Body, Refusal> {
    let plate = make_box([-20.0, -15.0, 0.0], [40.0, 30.0, 4.0])?;
    // the window's cutter, a box with its vertical edges rounded, runs past both faces
    let window = rounded_box([-10.0, -7.0, -1.0], [20.0, 14.0, 6.0], 2.0)?;
    offset_rounded(&cut(plate, window)?, 3.0)
}
