//! The oblate spheroid of `geometry/oblate-spheroid`, (x² + y²)/36 + z²/9 ≤ 1, grown by 1: every
//! point within 1 of it, bounded by its one face moved 1 out along its normal. The spheroid has no
//! edge to round, and this is the kernel's offset of the whole solid with its convex edges
//! rounded, as `offset/nurbs-ball-grown` is.
use crate::cases::geometry::oblate_spheroid;
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    offset_rounded(&oblate_spheroid::build()?, 1.0)
}
