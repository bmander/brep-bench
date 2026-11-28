//! A drum, the cylinder of radius 10 on the z axis from z = 0 to z = 10, with a groove round its
//! side from z = 4.2 to z = 5.8 cut to radius 7, grown by 1: every point within 1 of it. The
//! groove, narrower than 2, fills but for a shallow notch, SolidCraft's offset of the whole solid
//! (`offset_solid`), its convex edges rounded.
use crate::solids::*;

pub fn build() -> R<Body> {
    let drum = zcyl(0.0, 0.0, 10.0, 0.0, 10.0)?;
    // the groove: a ring from radius 7 out past the side
    let ring = cut(&zcyl(0.0, 0.0, 11.0, 4.2, 5.8)?, &zcyl(0.0, 0.0, 7.0, 4.0, 6.0)?)?;
    let grooved = cut(&drum, &ring)?;
    solidcraft_kernel::offset_solid(&grooved, 1.0, true)
}
