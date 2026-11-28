//! A block, the cylinder of radius 10 on the z axis from z = 0 to z = 8, with a blind bore of
//! radius 5 from z = 2 up through its top, the concave edge round the bore's floor filleted to
//! radius 2, grown by 3: every point within 3 of it, SolidCraft's offset of the whole solid
//! (`offset_solid`), its convex edges rounded.
use super::rounded_puck_shrunk_by_its_rounds::round_circles;
use crate::solids::*;

pub fn build() -> R<Body> {
    let block = zcyl(0.0, 0.0, 10.0, 0.0, 8.0)?;
    // the bore runs on past the top
    let bore = zcyl(0.0, 0.0, 5.0, 2.0, 9.0)?;
    let filleted = round_circles(&cut(&block, &bore)?, &[(5.0, 2.0)], 2.0)?;
    solidcraft_kernel::offset_solid(&filleted, 3.0, true)
}
