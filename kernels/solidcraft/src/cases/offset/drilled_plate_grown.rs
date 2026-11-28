//! The plate from (−10, −10, 0) to (10, 10, 4) with a hole of radius 3 drilled through it along
//! the z axis, grown by 1: every point within 1 of it, SolidCraft's offset of the whole solid
//! (`offset_solid`), its convex edges rounded. The plate grows as a box does, and the hole narrows
//! to radius 2, flaring at its rims into the inner quarters of tori.
use crate::solids::*;

pub fn build() -> R<Body> {
    let plate = block((-10.0, -10.0, 0.0), (10.0, 10.0, 4.0))?;
    // the drill runs past both faces
    let drill = zcyl(0.0, 0.0, 3.0, -1.0, 5.0)?;
    let drilled = cut(&plate, &drill)?;
    solidcraft_kernel::offset_solid(&drilled, 1.0, true)
}
