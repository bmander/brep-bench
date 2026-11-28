//! The plate from (−30, −30, 0) to (30, 30, 4) with a hole of radius 20 drilled through it along
//! the z axis, grown by 20, the hole's radius: every point within 20 of it, SolidCraft's offset of
//! the whole solid (`offset_solid`), its convex edges rounded. The hole closes onto its axis, and
//! over each end the grown face dips to a cusp, the inside of a horn torus about the rim.
use crate::solids::*;

pub fn build() -> R<Body> {
    let plate = block((-30.0, -30.0, 0.0), (30.0, 30.0, 4.0))?;
    // the drill runs past both faces
    let drill = zcyl(0.0, 0.0, 20.0, -1.0, 5.0)?;
    solidcraft_kernel::offset_solid(&cut(&plate, &drill)?, 20.0, true)
}
