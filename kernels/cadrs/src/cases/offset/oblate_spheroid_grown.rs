//! The solid oblate spheroid of `geometry/oblate-spheroid`, (x² + y²)/36 + z²/9 ≤ 1, grown by 1:
//! every point within 1 of it, bounded by the spheroid's surface moved 1 out along its normals
//! (not an ellipsoid).
//!
//! The spheroid as `geometry/oblate-spheroid` makes it (the half ellipse revolved); then cadrs's
//! offset with rounded edges.
use crate::cases::geometry::r#oblate_spheroid;
use crate::shared::offset::grown;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let spheroid = oblate_spheroid::build(k)?;
    grown(k, spheroid, 1.0)
}
