//! A napkin ring, the ball of radius 5 centred on the origin with a hole of radius 3 bored through
//! it along z, grown by 1: every point within 1 of it, its sphere moved out to radius 6, its bore
//! in to radius 2, its rims pieces of tori, SolidCraft's offset of the whole solid
//! (`offset_solid`), its convex edges rounded.
use crate::solids::*;

pub fn build() -> R<Body> {
    let ball = solidcraft_kernel::sphere(Vec3::ZERO, 5.0)?;
    let bore = zcyl(0.0, 0.0, 3.0, -6.0, 6.0)?;
    let ring = cut(&ball, &bore)?;
    solidcraft_kernel::offset_solid(&ring, 1.0, true)
}
