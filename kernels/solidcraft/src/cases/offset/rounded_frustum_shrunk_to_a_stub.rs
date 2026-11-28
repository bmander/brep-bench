//! The frustum of `offset/rounded-frustum-shrunk-past-its-rounds`, both rims rounded to radius
//! 1.5, shrunk by 2.5: every point at least 2.5 from its outside, SolidCraft's offset of the whole
//! solid inward (`offset_solid`, rounded), the moved faces that cross trimmed and merged.
use super::rounded_frustum_shrunk_past_its_rounds::frustum_rounded;
use crate::solids::*;

pub fn build() -> R<Body> {
    solidcraft_kernel::offset_solid(&frustum_rounded(&[(6.0, 0.0), (3.0, 6.0)])?, -2.5, true)
}
