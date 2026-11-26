//! The oblate spheroid (x² + y²)/36 + z²/9 ≤ 1 of `geometry/oblate-spheroid`, grown by 1 to its
//! parallel body: the kernel's offset of the solid of revolution.
use crate::cases::geometry::r#oblate_spheroid;
use crate::solids::offset::parallel;
use crate::Refusal;
use brep_kernel::BrepSolid;

pub fn build() -> Result<BrepSolid, Refusal> {
    Ok(parallel(&oblate_spheroid::build()?, 1.0)?)
}
