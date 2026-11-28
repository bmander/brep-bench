//! The plate from (−20, −15, 0) to (20, 15, 4) with a window cut through it, the rectangle from
//! (−10, −7) to (10, 7) with its corners rounded to radius 2, grown by 3, more than the corners'
//! radius: every point within 3 of it, SolidCraft's offset of the whole solid (`offset_solid`),
//! its convex edges rounded.
use super::pocketed_block_grown_past_its_corners::rounded_box;
use crate::solids::*;

pub fn build() -> R<Body> {
    let plate = block((-20.0, -15.0, 0.0), (20.0, 15.0, 4.0))?;
    // the window's cutter, a box with its vertical edges rounded, runs past both faces
    let window = rounded_box((-10.0, -7.0, -1.0), (10.0, 7.0, 5.0), 2.0)?;
    solidcraft_kernel::offset_solid(&cut(&plate, &window)?, 3.0, true)
}
