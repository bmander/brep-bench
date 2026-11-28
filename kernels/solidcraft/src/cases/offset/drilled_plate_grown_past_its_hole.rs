//! The plate from (−10, −10, 0) to (10, 10, 4) with a hole of radius 3 drilled through it along
//! the z axis, grown by 4, more than the hole's radius: every point within 4 of it, SolidCraft's
//! offset of the whole solid (`offset_solid`), its convex edges rounded. The plate grows as a box
//! does, the hole fills, and over each end of it the grown face dips in a dimple, the inside of a
//! spindle torus about the rim.
use crate::solids::*;

pub fn build() -> R<Body> {
    let plate = block((-10.0, -10.0, 0.0), (10.0, 10.0, 4.0))?;
    // the drill runs past both faces
    let drill = zcyl(0.0, 0.0, 3.0, -1.0, 5.0)?;
    let drilled = cut(&plate, &drill)?;
    solidcraft_kernel::offset_solid(&drilled, 4.0, true)
}
