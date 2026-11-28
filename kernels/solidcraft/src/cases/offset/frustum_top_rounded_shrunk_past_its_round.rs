//! The cone frustum on the z axis from radius 6 at z = 0 to radius 3 at z = 6, its top rim rounded
//! to radius 1.5 and its bottom rim sharp, shrunk by 2: every point at least 2 from its outside,
//! SolidCraft's offset of the whole solid inward (`offset_solid`, rounded), the moved faces that
//! cross trimmed and merged.
use super::rounded_frustum_shrunk_past_its_rounds::frustum_rounded;
use crate::solids::*;

pub fn build() -> R<Body> {
    solidcraft_kernel::offset_solid(&frustum_rounded(&[(3.0, 6.0)])?, -2.0, true)
}
