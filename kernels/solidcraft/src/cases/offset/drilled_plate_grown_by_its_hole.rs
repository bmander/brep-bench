//! The plate from (−10, −10, 0) to (10, 10, 4) with a hole of radius 3 drilled through it along
//! the z axis, grown by 3, the hole's radius: every point within 3 of it, SolidCraft's offset of
//! the whole solid (`offset_solid`), its convex edges rounded. The plate grows as a box does, the
//! hole closes onto its axis, and over each end of it the grown face dips in a dimple, the inside
//! of a horn torus about the rim.
use crate::solids::*;

pub fn build() -> R<Body> {
    grown(1.0)
}

/// The plate, its hole and the growth with every length `s` times as long.
pub fn grown(s: f64) -> R<Body> {
    let plate = block((-10.0 * s, -10.0 * s, 0.0), (10.0 * s, 10.0 * s, 4.0 * s))?;
    // the drill runs past both faces
    let drill = zcyl(0.0, 0.0, 3.0 * s, -1.0 * s, 5.0 * s)?;
    let drilled = cut(&plate, &drill)?;
    solidcraft_kernel::offset_solid(&drilled, 3.0 * s, true)
}
