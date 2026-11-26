//! The solid oblate spheroid of `geometry/oblate-spheroid`, (x² + y²)/36 + z²/9 ≤ 1, hollowed to a
//! closed wall 1 thick with no opening: the cavity bounded by the spheroid's surface moved 1
//! inward along its normals (not an ellipsoid).
//!
//! The spheroid as `geometry/oblate-spheroid` makes it (the half ellipse revolved); cadrs's Shell
//! feature with Hollow set (no face removed).
use crate::cases::geometry::r#oblate_spheroid;
use crate::shared::shell::hollow;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let spheroid = oblate_spheroid::build(k)?;
    hollow(k, spheroid, 1.0)
}
