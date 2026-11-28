//! The block from (−20, −15, 0) to (20, 15, 10) with a pocket cut down from its top to a floor at
//! z = 4, the rectangle from (−10, −7) to (10, 7) with its corners rounded to radius 2, grown by 3,
//! more than the corners' radius: every point within 3 of it, SolidCraft's offset of the whole
//! solid (`offset_solid`), its convex edges rounded.
use crate::solids::*;

/// The box from `a` to `b` with its four vertical edges, each picked at its middle, rounded to
/// `radius`.
pub fn rounded_box(a: (f64, f64, f64), b: (f64, f64, f64), radius: f64) -> R<Body> {
    let z = (a.2 + b.2) / 2.0;
    let verticals = [v3(a.0, a.1, z), v3(b.0, a.1, z), v3(b.0, b.1, z), v3(a.0, b.1, z)];
    solidcraft_kernel::fillet(&block(a, b)?, &verticals, radius)
}

pub fn build() -> R<Body> {
    let block = block((-20.0, -15.0, 0.0), (20.0, 15.0, 10.0))?;
    // the pocket's cutter, a box with its vertical edges rounded, runs on past the top
    let pocket = rounded_box((-10.0, -7.0, 4.0), (10.0, 7.0, 11.0), 2.0)?;
    solidcraft_kernel::offset_solid(&cut(&block, &pocket)?, 3.0, true)
}
