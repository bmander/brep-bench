//! The block from (−10, −10, 0) to (10, 10, 10) with a blind bore of radius 3 down its z axis from
//! the top to a floor at z = 2, grown by 4, more than the bore's radius: every point within 4 of
//! it, SolidCraft's offset of the whole solid (`offset_solid`), its convex edges rounded.
use crate::solids::*;

pub fn build() -> R<Body> {
    let block = block((-10.0, -10.0, 0.0), (10.0, 10.0, 10.0))?;
    // the bore runs on past the top
    let bore = zcyl(0.0, 0.0, 3.0, 2.0, 11.0)?;
    solidcraft_kernel::offset_solid(&cut(&block, &bore)?, 4.0, true)
}
