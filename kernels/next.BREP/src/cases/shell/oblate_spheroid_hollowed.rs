//! The oblate spheroid (x² + y²)/36 + z²/9 ≤ 1 of `geometry/oblate-spheroid`, hollowed by the
//! kernel's closed shell to a wall 1 thick, as `shell/sphere-hollowed` hollows its ball.
use crate::cases::geometry::r#oblate_spheroid;
use crate::Refusal;
use brep_kernel::{offset_shell_closed, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    Ok(offset_shell_closed(&oblate_spheroid::build()?, 1.0)?.solid)
}
