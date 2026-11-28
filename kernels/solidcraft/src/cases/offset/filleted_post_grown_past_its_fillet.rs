//! A plate, the cylinder of radius 8 on the z axis from z = −4 to z = 0, with a post on it, the
//! cylinder of radius 3 from z = 0 to z = 8, the concave edge where they meet filleted to radius
//! 2, grown by 3: every point within 3 of it, SolidCraft's offset of the whole solid
//! (`offset_solid`), its convex edges rounded.
use super::rounded_puck_shrunk_by_its_rounds::round_circles;
use crate::solids::*;

pub fn build() -> R<Body> {
    let plate = zcyl(0.0, 0.0, 8.0, -4.0, 0.0)?;
    let post = zcyl(0.0, 0.0, 3.0, 0.0, 8.0)?;
    let filleted = round_circles(&union(&plate, &post)?, &[(3.0, 0.0)], 2.0)?;
    solidcraft_kernel::offset_solid(&filleted, 3.0, true)
}
