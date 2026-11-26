//! The prolate spheroid (x² + y²)/9 + z²/36 ≤ 1, the half ellipse of the xz plane and its axis
//! turned a whole turn about the z axis, hollowed by the kernel's closed shell to a wall 1 thick,
//! as `shell/sphere-hollowed` hollows its ball.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_ellipse_arc, make_line, offset_shell_closed, revolve_profile_brep, BrepSolid};
use std::f64::consts::{FRAC_PI_2, TAU};

pub fn build() -> Result<BrepSolid, Refusal> {
    let (o, x, z) = (p(0.0, 0.0, 0.0), p(1.0, 0.0, 0.0), p(0.0, 0.0, 1.0));
    let half = [
        make_ellipse_arc(o, x, z, 3.0, 6.0, -FRAC_PI_2, FRAC_PI_2)?,
        make_line(p(0.0, 0.0, 6.0), p(0.0, 0.0, -6.0))?,
    ];
    let spheroid = revolve_profile_brep(&half, o, z, TAU)?;
    Ok(offset_shell_closed(&spheroid, 1.0)?.solid)
}
