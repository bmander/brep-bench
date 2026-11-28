//! The plate from (−10, −10, 0) to (10, 10, 4) with a round recess of radius 3 sunk 1 into its top
//! about the z axis, grown by 4, more than the recess's radius: every point within 4 of it,
//! SolidCraft's offset of the whole solid (`offset_solid`), its convex edges rounded.
use crate::solids::*;

pub fn build() -> R<Body> {
    let plate = block((-10.0, -10.0, 0.0), (10.0, 10.0, 4.0))?;
    // the recess's cutter runs on past the top
    let recess = zcyl(0.0, 0.0, 3.0, 3.0, 5.0)?;
    solidcraft_kernel::offset_solid(&cut(&plate, &recess)?, 4.0, true)
}
